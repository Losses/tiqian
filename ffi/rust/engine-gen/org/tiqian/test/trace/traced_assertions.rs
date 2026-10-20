use crate::org::tiqian::clreq::bopomofo_reading::BopomofoReading;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::glue_side::GlueSide;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteType;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace::TestTrace;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError;
use crate::org::tiqian::test::trace::trace_field::TraceField;
use crate::runtime::exception::Exception;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsFailFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for TracedAssertionsFailFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsFailFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TracedAssertionsFailFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsFailFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsFailFault) -> Self {
        match value {
            TracedAssertionsFailFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsFailFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsFailFault) -> Self {
        match value {
            TracedAssertionsFailFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsFailFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsFailFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsFailFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsFailFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertTrueFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertTrueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertTrueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertTrueFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertTrueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertTrueFault) -> Self {
        match value {
            TracedAssertionsAssertTrueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertTrueFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertTrueFault) -> Self {
        match value {
            TracedAssertionsAssertTrueFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertTrueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertTrueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertTrueFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertTrueFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertNullRenderedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertNullRenderedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertNullRenderedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertNullRenderedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertNullRenderedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertNullRenderedFault) -> Self {
        match value {
            TracedAssertionsAssertNullRenderedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertNullRenderedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertNullRenderedFault) -> Self {
        match value {
            TracedAssertionsAssertNullRenderedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertNullRenderedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertNullRenderedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertNullRenderedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertNullRenderedFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertNotNullRenderedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertNotNullRenderedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertNotNullRenderedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertNotNullRenderedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertNotNullRenderedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertNotNullRenderedFault) -> Self {
        match value {
            TracedAssertionsAssertNotNullRenderedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertNotNullRenderedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertNotNullRenderedFault) -> Self {
        match value {
            TracedAssertionsAssertNotNullRenderedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertNotNullRenderedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertNotNullRenderedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertNotNullRenderedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertNotNullRenderedFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertFalseFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertFalseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertFalseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertFalseFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertFalseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertFalseFault) -> Self {
        match value {
            TracedAssertionsAssertFalseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertFalseFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertFalseFault) -> Self {
        match value {
            TracedAssertionsAssertFalseFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertFalseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertFalseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertFalseFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertFalseFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertFailsWithNoSuchElementFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for TracedAssertionsAssertFailsWithNoSuchElementFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertFailsWithNoSuchElementFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertFailsWithNoSuchElementFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertFailsWithNoSuchElementFault) -> Self {
        match value {
            TracedAssertionsAssertFailsWithNoSuchElementFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertFailsWithNoSuchElementFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertFailsWithNoSuchElementFault) -> Self {
        match value {
            TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertFailsWithNoSuchElementFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertFailsWithNoSuchElementFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertFailsWithNoSuchElementFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertFailsWithFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for TracedAssertionsAssertFailsWithFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertFailsWithFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertFailsWithFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertFailsWithFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertFailsWithFault) -> Self {
        match value {
            TracedAssertionsAssertFailsWithFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertFailsWithFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertFailsWithFault) -> Self {
        match value {
            TracedAssertionsAssertFailsWithFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertFailsWithFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertFailsWithFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertFailsWithFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertFailsWithFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsTextRangeArrayFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsTextRangeArrayFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsTextRangeArrayFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsTextRangeArrayFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsTextRangeArrayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsTextRangeArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsTextRangeArrayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsTextRangeArrayFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsTextRangeArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsTextRangeArrayFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsTextRangeArrayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsTextRangeArrayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsTextRangeArrayFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsTextRangeArrayFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsStringArrayFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsStringArrayFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsStringArrayFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsStringArrayFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsStringArrayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsStringArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsStringArrayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsStringArrayFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsStringArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsStringArrayFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsStringArrayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsStringArrayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsStringArrayFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsStringArrayFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsStringFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsStringFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsStringFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsStringFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsStringFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsStringFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsStringFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsStringFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsStringFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsStringFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsStringFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsStringFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsStringFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsStringFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsRepairOptionArrayFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    FailFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsRepairOptionArrayFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsRepairOptionArrayFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsRepairOptionArrayFault::FailFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsRepairOptionArrayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsRepairOptionArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsRepairOptionArrayFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsRepairOptionArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsRepairOptionArrayFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsRepairOptionArrayFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TracedAssertionsAssertEqualsRepairOptionArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsRepairOptionArrayFault::FailFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsRepairOptionArrayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsRepairOptionArrayFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsRepairOptionArrayFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TracedAssertionsAssertEqualsRepairOptionArrayFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TracedAssertionsAssertEqualsRepairOptionArrayFault::FailFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsRenderedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsRenderedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsRenderedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsRenderedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsRenderedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsRenderedFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsRenderedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsRenderedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsRenderedFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsRenderedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsRenderedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsRenderedFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsQuoteTypeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsQuoteTypeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsQuoteTypeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsQuoteTypeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsQuoteTypeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsQuoteTypeFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsQuoteTypeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsQuoteTypeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsQuoteTypeFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsQuoteTypeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsQuoteTypeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsQuoteTypeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsQuoteTypeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsQuoteTypeFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsQuotePairArrayFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsQuotePairArrayFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsQuotePairArrayFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsQuotePairArrayFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsQuotePairArrayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsQuotePairArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsQuotePairArrayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsQuotePairArrayFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsQuotePairArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsQuotePairArrayFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsQuotePairArrayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsQuotePairArrayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsQuotePairArrayFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsQuotePairArrayFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsQuotePairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsQuotePairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsQuotePairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsQuotePairFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsQuotePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsQuotePairFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsQuotePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsQuotePairFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsQuotePairFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsQuotePairFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsQuotePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsQuotePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsQuotePairFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsQuotePairFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsPushInAllocationArrayFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    FailFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsPushInAllocationArrayFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsPushInAllocationArrayFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsPushInAllocationArrayFault::FailFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsPushInAllocationArrayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsPushInAllocationArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsPushInAllocationArrayFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsPushInAllocationArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsPushInAllocationArrayFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsPushInAllocationArrayFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TracedAssertionsAssertEqualsPushInAllocationArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsPushInAllocationArrayFault::FailFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsPushInAllocationArrayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsPushInAllocationArrayFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsPushInAllocationArrayFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TracedAssertionsAssertEqualsPushInAllocationArrayFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TracedAssertionsAssertEqualsPushInAllocationArrayFault::FailFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsPunctuationGluePlacementFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsPunctuationGluePlacementFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsPunctuationGluePlacementFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsPunctuationGluePlacementFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsPunctuationGluePlacementFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsPunctuationGluePlacementFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsPunctuationGluePlacementFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsPunctuationGluePlacementFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsPunctuationGluePlacementFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsPunctuationGluePlacementFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsPunctuationGluePlacementFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsPunctuationGluePlacementFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsPunctuationGluePlacementFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsPunctuationGluePlacementFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsPunctuationClassFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsPunctuationClassFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsPunctuationClassFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsPunctuationClassFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsPunctuationClassFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsPunctuationClassFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsPunctuationClassFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsPunctuationClassFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsPunctuationClassFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsPunctuationClassFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsPunctuationClassFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsPunctuationClassFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsPunctuationClassFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsPunctuationClassFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsNullableStringFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsNullableStringFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsNullableStringFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsNullableStringFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableStringFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsNullableStringFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableStringFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableStringFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsNullableStringFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableStringFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsNullableStringFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsNullableStringFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsNullableStringFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsNullableStringFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsNullableRepairOptionFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsNullableRepairOptionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsNullableRepairOptionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsNullableRepairOptionFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableRepairOptionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsNullableRepairOptionFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableRepairOptionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableRepairOptionFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsNullableRepairOptionFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableRepairOptionFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsNullableRepairOptionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsNullableRepairOptionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsNullableRepairOptionFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsNullableRepairOptionFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsNullableIntFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsNullableIntFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsNullableIntFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsNullableIntFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableIntFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsNullableIntFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableIntFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableIntFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsNullableIntFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableIntFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsNullableIntFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsNullableIntFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsNullableIntFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsNullableIntFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsNullableFloatFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsNullableFloatFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsNullableFloatFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsNullableFloatFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableFloatFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsNullableFloatFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableFloatFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsNullableFloatFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsNullableFloatFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsNullableFloatFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsNullableFloatFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsNullableFloatFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsNullableFloatFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsNullableFloatFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsKinsokuLevelFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsKinsokuLevelFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsKinsokuLevelFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsKinsokuLevelFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsKinsokuLevelFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsKinsokuLevelFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsKinsokuLevelFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsKinsokuLevelFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsKinsokuLevelFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsKinsokuLevelFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsKinsokuLevelFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsKinsokuLevelFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsKinsokuLevelFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsKinsokuLevelFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsIntSetUnorderedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsIntSetUnorderedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsIntSetUnorderedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsIntSetUnorderedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntSetUnorderedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsIntSetUnorderedFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntSetUnorderedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntSetUnorderedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsIntSetUnorderedFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntSetUnorderedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsIntSetUnorderedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsIntSetUnorderedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsIntSetUnorderedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsIntSetUnorderedFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsIntSetFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsIntSetFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsIntSetFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsIntSetFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntSetFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsIntSetFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntSetFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntSetFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsIntSetFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntSetFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsIntSetFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsIntSetFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsIntSetFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsIntSetFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsIntRangeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsIntRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsIntRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsIntRangeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntRangeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntRangeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsIntRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsIntRangeFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsIntArrayFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsIntArrayFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsIntArrayFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsIntArrayFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntArrayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsIntArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntArrayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntArrayFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsIntArrayFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntArrayFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsIntArrayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsIntArrayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsIntArrayFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsIntArrayFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsIntFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsIntFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsIntFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsIntFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsIntFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIntFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsIntFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIntFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsIntFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsIntFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsIntFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsIntFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsIcFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsIcFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsIcFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsIcFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIcFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsIcFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIcFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsIcFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsIcFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsIcFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsIcFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsIcFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsIcFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsIcFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsHangingPunctuationStyleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsHangingPunctuationStyleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsHangingPunctuationStyleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsHangingPunctuationStyleFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsHangingPunctuationStyleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsHangingPunctuationStyleFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsHangingPunctuationStyleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsHangingPunctuationStyleFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsHangingPunctuationStyleFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsHangingPunctuationStyleFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsHangingPunctuationStyleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsHangingPunctuationStyleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsHangingPunctuationStyleFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsHangingPunctuationStyleFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsGlueSideFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsGlueSideFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsGlueSideFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsGlueSideFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsGlueSideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsGlueSideFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsGlueSideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsGlueSideFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsGlueSideFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsGlueSideFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsGlueSideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsGlueSideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsGlueSideFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsGlueSideFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsFontRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsFontRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsFontRoleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsFontRoleFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFontRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsFontRoleFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFontRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFontRoleFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsFontRoleFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFontRoleFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsFontRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsFontRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsFontRoleFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsFontRoleFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsFloatToleranceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsFloatToleranceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsFloatToleranceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsFloatToleranceFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFloatToleranceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsFloatToleranceFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFloatToleranceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFloatToleranceFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsFloatToleranceFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFloatToleranceFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsFloatToleranceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsFloatToleranceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsFloatToleranceFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsFloatToleranceFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsFloatFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsFloatFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsFloatFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsFloatFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFloatFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsFloatFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFloatFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFloatFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsFloatFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFloatFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsFloatFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsFloatFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsFloatFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsFloatFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsEnumFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsEnumFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsEnumFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsEnumFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsEnumFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsEnumFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsEnumFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsEnumFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsEnumFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsEnumFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsEnumFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsEnumFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsEnumFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsEnumFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsEastAsianSpacingEdgesFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsClreqProfileFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsClreqProfileFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsClreqProfileFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsClreqProfileFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsClreqProfileFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsClreqProfileFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsClreqProfileFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsClreqProfileFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsClreqProfileFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsClreqProfileFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsClreqProfileFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsClreqProfileFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsClreqProfileFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsClreqProfileFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsBopomofoToneFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsBopomofoToneFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsBopomofoToneFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsBopomofoToneFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsBopomofoToneFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsBopomofoToneFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsBopomofoToneFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsBopomofoToneFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsBopomofoToneFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsBopomofoToneFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsBopomofoToneFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsBopomofoToneFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsBopomofoToneFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsBopomofoToneFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsBopomofoReadingFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsBopomofoReadingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsBopomofoReadingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsBopomofoReadingFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsBopomofoReadingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsBopomofoReadingFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsBopomofoReadingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsBopomofoReadingFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsBopomofoReadingFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsBopomofoReadingFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsBopomofoReadingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsBopomofoReadingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsBopomofoReadingFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsBopomofoReadingFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsBoolFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsBoolFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsBoolFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsBoolFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsBoolFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsBoolFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsBoolFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsBoolFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsBoolFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsBoolFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsBoolFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsBoolFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsBoolFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsBoolFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsAssertEqualsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
}
impl std::fmt::Display for TracedAssertionsAssertEqualsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracedAssertionsAssertEqualsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TracedAssertionsAssertEqualsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TracedAssertionsAssertEqualsFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TracedAssertionsAssertEqualsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TracedAssertionsAssertEqualsFault) -> Self {
        match value {
            TracedAssertionsAssertEqualsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TracedAssertionsAssertEqualsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TracedAssertionsAssertEqualsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TracedAssertionsAssertEqualsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TracedAssertionsAssertEqualsFault::TraceAssertionErrorFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct TracedAssertions;

impl TracedAssertions {
    pub fn traced_assertions_assert_equals(expected: u32, actual: u32, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_render_int(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_int(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option1) => __option1.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_bool(expected: bool, actual: bool, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_render_bool(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_bool(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option3) => __option3.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_repair_option(expected: Option<RepairOption>, actual: Option<RepairOption>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), match &(expected) { None => UString::from("-"), Some(__option12) => TracedAssertions::traced_assertions_render_repair_option((*__option12).clone()).to_ustring() }.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), match &(actual) { None => UString::from("-"), Some(__option15) => TracedAssertions::traced_assertions_render_repair_option((*__option15).clone()).to_ustring() }.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option17) => __option17.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_string(expected: Option<UString>, actual: Option<UString>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), match &(expected) { None => UString::from("-"), Some(__option26) => TestTraceRender::test_trace_render_render_string(__option26).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_ustring() }.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), match &(actual) { None => UString::from("-"), Some(__option29) => TestTraceRender::test_trace_render_render_string(__option29).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_ustring() }.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option31) => __option31.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_string(expected: &UStr, actual: &UStr, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_render_string(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_string(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option33) => __option33.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_ic(expected: Ic, actual: Ic, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from((expected).to_string().as_str()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from((actual).to_string().as_str()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option35) => __option35.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_enum<T: Clone + std::fmt::Debug + PartialEq>(expected: &T, actual: &T, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(format!("{}", format!("{:?}", expected)).as_str()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(format!("{}", format!("{:?}", actual)).as_str()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option37) => __option37.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_east_asian_spacing_edges(expected: EastAsianSpacingEdges, actual: EastAsianSpacingEdges, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(format!("{}", expected.to_string()).as_str()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(format!("{}", actual.to_string()).as_str()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected.leading != actual.leading || expected.trailing != actual.trailing || expected.contains_wide != actual.contains_wide {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option39) => __option39.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_int_array(expected: &[u32], actual: &[u32], message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TracedAssertions::traced_assertions_render_ints(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TracedAssertions::traced_assertions_render_ints(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option41) => __option41.to_ustring() }).to_ustring()), None)?;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if expected[usize::try_from(i).unwrap_or(0)] != actual[usize::try_from(i).unwrap_or(0)] {
                let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option43) => __option43.to_ustring() }).to_ustring()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_ints(values: &[u32]) -> Result<UString, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("[").encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                buf.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !{ let mut __s = UString::new(); __s += &(UString::from("")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(values[usize::try_from(i).unwrap_or(0)])).as_str())); __s }.is_empty() {
                    if !{ let mut __s = UString::new(); __s += &(UString::from("")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(values[usize::try_from(i).unwrap_or(0)])).as_str())); __s }.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            buf.extend({ let mut __s = UString::new(); __s += &(UString::from("")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(values[usize::try_from(i).unwrap_or(0)])).as_str())); __s }.encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_string_array(expected: &[UString], actual: &[UString], message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_render_string_array(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_string_array(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !TracedAssertions::traced_assertions_same_string_array(&expected, &actual) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option45) => __option45.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_bopomofo_tone(expected: BopomofoTone, actual: BopomofoTone, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(actual.name()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option47) => __option47.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_font_role(expected: FontRole, actual: Option<FontRole>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), match &(actual) { None => UString::from("null"), Some(__option53) => TracedAssertions::traced_assertions_render_font_role(*__option53).to_ustring() }.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if match &(actual) { None => true, Some(__option54) => Some(expected) != Some(*__option54) } {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option56) => __option56.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_font_role(value: FontRole) -> UString {
        return UString::from(value.name());
    }

    pub fn traced_assertions_assert_equals_quote_type(expected: QuoteType, actual: QuoteType, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(actual.name()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option58) => __option58.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_quote_pair(expected: QuotePair, actual: QuotePair, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(format!("{}", expected.to_string()).as_str()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(format!("{}", actual.to_string()).as_str()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected.open_index != actual.open_index || expected.close_index != actual.close_index || expected.quote_type != actual.quote_type {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option60) => __option60.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_quote_pairs(values: &Vec<QuotePair>) -> Result<UString, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("[").encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                buf.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !(values[usize::try_from(i).unwrap_or(0)]).clone().to_string().is_empty() {
                    if !(values[usize::try_from(i).unwrap_or(0)]).clone().to_string().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            buf.extend((values[usize::try_from(i).unwrap_or(0)]).clone().to_string().encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_quote_pair_array(expected: &Vec<QuotePair>, actual: &Vec<QuotePair>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TracedAssertions::traced_assertions_render_quote_pairs(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TracedAssertions::traced_assertions_render_quote_pairs(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option62) => __option62.to_ustring() }).to_ustring()), None)?;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if expected[usize::try_from(i).unwrap_or(0)].open_index != actual[usize::try_from(i).unwrap_or(0)].open_index || expected[usize::try_from(i).unwrap_or(0)].close_index != actual[usize::try_from(i).unwrap_or(0)].close_index || expected[usize::try_from(i).unwrap_or(0)].quote_type != actual[usize::try_from(i).unwrap_or(0)].quote_type {
                let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option64) => __option64.to_ustring() }).to_ustring()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_int(expected: Option<u32>, actual: Option<u32>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), match &(expected) { None => UString::from("-"), Some(__option73) => TestTraceRender::test_trace_render_render_int(*__option73).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_ustring() }.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), match &(actual) { None => UString::from("-"), Some(__option76) => TestTraceRender::test_trace_render_render_int(*__option76).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_ustring() }.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option78) => __option78.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_float(expected: Option<f64>, actual: Option<f64>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), match &(expected) { None => UString::from("-"), Some(__option87) => TestTraceRender::test_trace_render_render_float(*__option87).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_ustring() }.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), match &(actual) { None => UString::from("-"), Some(__option90) => TestTraceRender::test_trace_render_render_float(*__option90).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_ustring() }.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option92) => __option92.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_text_range_array(expected: &Vec<TextRange>, actual: &Vec<TextRange>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let expected_text = TracedAssertions::traced_assertions_render_text_range_array(&expected);
        let actual_text = TracedAssertions::traced_assertions_render_text_range_array(&actual);
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), expected_text.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), actual_text.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option94) => __option94.to_ustring() }).to_ustring()), None)?;
        }
        for i in 0..match u32::try_from(expected.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if expected[usize::try_from(i).unwrap_or(0)].start != actual[usize::try_from(i).unwrap_or(0)].start || expected[usize::try_from(i).unwrap_or(0)].end != actual[usize::try_from(i).unwrap_or(0)].end {
                let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option96) => __option96.to_ustring() }).to_ustring()), None)?;
            }
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_text_range_array(ranges: &Vec<TextRange>) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (ranges[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn traced_assertions_assert_equals_int_range(expected: IntRange, actual: IntRange, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TracedAssertions::traced_assertions_render_int_range((expected).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TracedAssertions::traced_assertions_render_int_range((actual).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected.start != actual.start || expected.end != actual.end {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option98) => __option98.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_int_range(r: IntRange) -> Result<UString, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("[").encode_utf16());
        let mut i = r.start;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((r.end) as i32).to_ne_bytes()) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((r.start) as i32).to_ne_bytes())) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                buf.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_render_int(i)?.is_empty() {
                    if !TestTraceRender::test_trace_render_render_int(i)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            buf.extend(TestTraceRender::test_trace_render_render_int(i)?.encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_int_set(expected: SortedSetTable<u32>, actual: SortedSetTable<u32>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TracedAssertions::traced_assertions_render_int_set((expected).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TracedAssertions::traced_assertions_render_int_set((actual).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::from_ne_bytes(((expected.size()) as u32).to_ne_bytes()) != u32::from_ne_bytes(((actual.size()) as u32).to_ne_bytes()) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option100) => __option100.to_ustring() }).to_ustring()), None)?;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((expected.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            if expected.at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())) != actual.at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())) {
                let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option102) => __option102.to_ustring() }).to_ustring()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_int_set(values: SortedSetTable<u32>) -> Result<UString, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("[").encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((values.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                buf.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_render_int(values.at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())))?.is_empty() {
                    if !TestTraceRender::test_trace_render_render_int(values.at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())))?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            buf.extend(TestTraceRender::test_trace_render_render_int(values.at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())))?.encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_int_set_unordered(expected: &Vec<u32>, actual: &Vec<u32>, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TracedAssertions::traced_assertions_render_int_list_in_given_order(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TracedAssertions::traced_assertions_render_int_list_in_given_order(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let mut expected_set: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            expected_set.put(&(expected[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        let mut actual_set: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            actual_set.put(&(actual[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        let e: SortedSetTable<u32> = expected_set.clone().build();
        let a: SortedSetTable<u32> = actual_set.clone().build();
        if u32::from_ne_bytes(((e.size()) as u32).to_ne_bytes()) != u32::from_ne_bytes(((a.size()) as u32).to_ne_bytes()) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option104) => __option104.to_ustring() }).to_ustring()), None)?;
        }
        i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((e.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            if e.at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())) != a.at(i32::from_ne_bytes(((i) as i32).to_ne_bytes())) {
                let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option106) => __option106.to_ustring() }).to_ustring()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_int_list_in_given_order(values: &Vec<u32>) -> Result<UString, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("[").encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                buf.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_render_int(values[usize::try_from(i).unwrap_or(0)])?.is_empty() {
                    if !TestTraceRender::test_trace_render_render_int(values[usize::try_from(i).unwrap_or(0)])?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            buf.extend(TestTraceRender::test_trace_render_render_int(values[usize::try_from(i).unwrap_or(0)])?.encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_repair_option_array(expected: &Vec<RepairOption>, actual: &Vec<RepairOption>, message: Option<UString>) -> Result<(), TracedAssertionsAssertEqualsRepairOptionArrayFault> {
        let expected_text = TracedAssertions::traced_assertions_render_repair_option_list(&expected).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?;
        let actual_text = TracedAssertions::traced_assertions_render_repair_option_list(&actual).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), expected_text.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), actual_text.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?;
        if expected_text != actual_text {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option108) => __option108.to_ustring() }).to_ustring()), None).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::FailFault(e))?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_repair_option_list(values: &Vec<RepairOption>) -> Result<UString, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("[").encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                buf.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TracedAssertions::traced_assertions_render_repair_option((values[usize::try_from(i).unwrap_or(0)]).clone()).is_empty() {
                    if !TracedAssertions::traced_assertions_render_repair_option((values[usize::try_from(i).unwrap_or(0)]).clone()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            buf.extend(TracedAssertions::traced_assertions_render_repair_option((values[usize::try_from(i).unwrap_or(0)]).clone()).encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub(crate) fn traced_assertions_render_repair_option(option: RepairOption) -> UString {
        return UString::from(format!("{}", option.to_string()).as_str());
    }

    pub(crate) fn traced_assertions_render_push_in_allocations(values: &Vec<PushInAllocation>) -> Result<UString, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("[").encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                buf.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !(values[usize::try_from(i).unwrap_or(0)]).clone().to_string().is_empty() {
                    if !(values[usize::try_from(i).unwrap_or(0)]).clone().to_string().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            buf.extend((values[usize::try_from(i).unwrap_or(0)]).clone().to_string().encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        buf.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_push_in_allocation_array(expected: &Vec<PushInAllocation>, actual: &Vec<PushInAllocation>, message: Option<UString>) -> Result<(), TracedAssertionsAssertEqualsPushInAllocationArrayFault> {
        let expected_text = TracedAssertions::traced_assertions_render_push_in_allocations(&expected).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?;
        let actual_text = TracedAssertions::traced_assertions_render_push_in_allocations(&actual).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), expected_text.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), actual_text.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?;
        if expected_text != actual_text {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected arrays to be equal."), Some(__option110) => __option110.to_ustring() }).to_ustring()), None).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::FailFault(e))?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_bopomofo_reading(expected: BopomofoReading, actual: BopomofoReading, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(format!("{}", expected.to_string()).as_str()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(format!("{}", actual.to_string()).as_str()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !TracedAssertions::traced_assertions_same_string_array(&expected.symbols, &actual.symbols) || expected.tone != actual.tone {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option112) => __option112.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_punctuation_class(expected: PunctuationClass, actual: PunctuationClass, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(actual.name()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option114) => __option114.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_glue_side(expected: GlueSide, actual: GlueSide, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(actual.name()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option116) => __option116.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_punctuation_glue_placement(expected: PunctuationGluePlacement, actual: PunctuationGluePlacement, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(actual.name()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option118) => __option118.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_kinsoku_level(expected: KinsokuLevel, actual: KinsokuLevel, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(actual.name()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option120) => __option120.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_hanging_punctuation_style(expected: HangingPunctuationStyle, actual: HangingPunctuationStyle, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), UString::from(expected.name()).as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), UString::from(actual.name()).as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option122) => __option122.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_clreq_profile(expected: ClreqProfile, actual: ClreqProfile, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_cap(UString::from(format!("{}", expected.to_string()).as_str()).as_ustr()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),

    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_cap(UString::from(format!("{}", actual.to_string()).as_str()).as_ustr()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !ClreqProfile::clreq_profile_same_profile((expected).clone(), (actual).clone()) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option124) => __option124.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_same_string_array(first: &[UString], second: &[UString]) -> bool {
        if u32::try_from((first.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((second.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((first.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if first[usize::try_from(index).unwrap_or(0)].clone() != (second[usize::try_from(index).unwrap_or(0)]).clone() {
                return false;
            }
            index = u32::wrapping_add(index, 1);
        }
        return true;
    }

    pub fn traced_assertions_f32_literal(value: f64) -> f64 {
        return TestHelpers::test_helpers_f32_literal(value);
    }

    pub fn traced_assertions_assert_equals_float(expected: f64, actual: f64, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_render_float(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_float(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option126) => __option126.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_float_tolerance(expected: f64, actual: f64, tolerance: f64, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113,45,116,111,108]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_render_float(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_float(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[116,111,108]), TestTraceRender::test_trace_render_render_float(tolerance).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != expected || actual != actual || ((expected - actual).abs()) > (tolerance) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal within tolerance."), Some(__option128) => __option128.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_rendered(expected: &UStr, actual: &UStr, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let expected_text = TestTraceRender::test_trace_render_cap(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let actual_text = TestTraceRender::test_trace_render_cap(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), expected_text.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), actual_text.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected_text != actual_text {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected rendered values to be equal."), Some(__option130) => __option130.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_int(expected: u32, actual: u32, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[101,113]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,112,101,99,116,101,100]), TestTraceRender::test_trace_render_render_int(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_int(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected values to be equal."), Some(__option132) => __option132.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_true(actual: bool, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[105,115,45,116,114,117,101]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_bool(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected value to be true."), Some(__option134) => __option134.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_false(actual: bool, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[105,115,45,102,97,108,115,101]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), TestTraceRender::test_trace_render_render_bool(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if actual {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected value to be false."), Some(__option136) => __option136.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_null_rendered(was_null: bool, rendered_actual: &UStr, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[110,117,108,108]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), rendered_actual)),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !was_null {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected value to be null."), Some(__option138) => __option138.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_not_null_rendered(was_not_null: bool, rendered_actual: &UStr, message: Option<UString>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[110,111,116,45,110,117,108,108]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), rendered_actual)),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !was_not_null {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Expected value to be non-null."), Some(__option140) => __option140.to_ustring() }).to_ustring()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_record_rendered_not_null(rendered_actual: &UStr, message: Option<UString>) -> Result<(), UStringFault> {
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[110,111,116,45,110,117,108,108]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[97,99,116,117,97,108]), rendered_actual)),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone())?,
])?;
        Ok(())
    }

    pub fn traced_assertions_assert_fails_with(message: Option<UString>, block: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static>) -> Result<TextRangeError, TracedAssertionsAssertFailsWithFault> {
        let mut caught: Option<TextRangeError> = None;
        let mut derived = false;
        let __outcome: Result<(), TextRangeError> = (|| {
            let __outcome1: Result<(), IllegalStateException> = (|| {
                block()?;
                Ok(())
            })();
            match __outcome1 {
                Ok(_) => {}
                Err(__caught) => {
                        let error = TextRangeError::Message { text: UString::from(format!("{}", __caught.message.clone()).as_str()) };
                        caught = Some(error.clone());
                        derived = true;
                }
            }
            Ok(())
        })();
        match __outcome {
            Ok(_) => {}
            Err(error) => {
                caught = Some(error.clone());
                derived = false;
            }
        }
        match &(caught) {
            Some(__option141) => {
                let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[114,97,105,115,101,115]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,99,101,112,116,105,111,110]), if derived { UString::from("IllegalStateException") } else { UString::from("TiqianIllegalArgumentException") }.as_ustr())),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[116,104,114,111,119,110]), TestTraceRender::test_trace_render_render_string(UString::from(format!("{}", format!("{}", __option141)).as_str()).as_ustr()).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?;
                return Ok((__option141).clone());
            }
            None => {
            }
        }
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[102,97,105,108]), &vec![
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?;
        return Err(TracedAssertionsAssertFailsWithFault::TraceAssertionErrorFault(TraceAssertionError::AssertionFailed { message: match &(message) { None => UString::from("Expected an exception."), Some(__option142) => __option142.to_ustring() }.to_ustring() }));
    }

    pub fn traced_assertions_assert_fails_with_no_such_element(message: Option<UString>, block: Arc<dyn Fn() -> Result<(), NoSuchElementError> + Send + Sync + 'static>) -> Result<NoSuchElementError, TracedAssertionsAssertFailsWithNoSuchElementFault> {
        let __outcome2: Result<(), NoSuchElementError> = (|| {
            block()?;
            Ok(())
        })();
        match __outcome2 {
            Ok(_) => {}
            Err(error) => {
                let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[114,97,105,115,101,115]), &vec![
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[101,120,99,101,112,116,105,111,110]), UStr::new(&[84,105,113,105,97,110,78,111,83,117,99,104,69,108,101,109,101,110,116,69,120,99,101,112,116,105,111,110]))),
    Some(TracedAssertions::traced_assertions_field(UStr::new(&[116,104,114,111,119,110]), TestTraceRender::test_trace_render_render_string(UString::from(format!("{}", format!("{}", error)).as_str()).as_ustr()).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?.as_ustr())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?;
                return Ok(error);
            }
        }
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[102,97,105,108]), &vec![
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?;
        return Err(TracedAssertionsAssertFailsWithNoSuchElementFault::TraceAssertionErrorFault(TraceAssertionError::AssertionFailed { message: match &(message) { None => UString::from("Expected an exception."), Some(__option143) => __option143.to_ustring() }.to_ustring() }));
    }

    pub fn traced_assertions_fail(message: Option<UString>, _cause: Option<Exception>) -> Result<(), TracedAssertionsFailFault> {
        let text = match &(message) { None => UString::from("Assertion failed."), Some(__option144) => __option144.to_ustring() };
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[102,97,105,108]), &vec![
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_ustring()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        return Err(TracedAssertionsFailFault::TraceAssertionErrorFault(TraceAssertionError::AssertionFailed { message: text.to_ustring() }));
    }

    pub fn traced_assertions_assert_does_not_throw<T: Clone>(block: Arc<dyn Fn() -> T + Send + Sync>) -> Result<T, UStringFault> {
        let result = block();
        let _ = TracedAssertions::traced_assertions_record_event(UStr::new(&[110,111,45,116,104,114,111,119]), &vec![])?;
        return Ok((result).clone());
    }

    pub(crate) fn traced_assertions_record_event(name: &UStr, fields: &Vec<Option<TraceField>>) -> Result<(), UStringFault> {
        if !TestTrace::TEST_TRACE_UPDATE_MODE {
            return Ok(());
        }
        let mut line = Vec::<u16>::new();
        if let Some(&unit) = line.last() {
            if unit >= 55296 && unit <= 56319 && !name.is_empty() {
                if !name.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        line.extend(name.encode_utf16());
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let current = (fields[usize::try_from(index).unwrap_or(0)]).clone();
            match &(current) {
                Some(__option145) => {
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(" ").is_empty() {
                            if !UString::from(" ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    line.extend(UString::from(" ").encode_utf16());
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !(__option145.key).to_ustring().is_empty() {
                            if !(__option145.key).to_ustring().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    line.extend((__option145.key).to_ustring().encode_utf16());
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("=").is_empty() {
                            if !UString::from("=").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    line.extend(UString::from("=").encode_utf16());
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_canonical_numbers((__option145.value).to_ustring().as_ustr())?.is_empty() {
                            if !TestTraceRender::test_trace_render_canonical_numbers((__option145.value).to_ustring().as_ustr())?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    line.extend(TestTraceRender::test_trace_render_canonical_numbers((__option145.value).to_ustring().as_ustr())?.encode_utf16());
                }
                None => {
                }
            }
            index = u32::wrapping_add(index, 1);
        }
        let rendered = UString::from_utf16(line.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(line[line.len() - 1]) })?;
        let recorder = TestTrace::test_trace_current_recorder();
        match &(recorder) {
            Some(__option146) => {
                let _ = __option146.record(rendered.as_ustr())?;
            }
            None => {
            }
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_field(key: &UStr, value: &UStr) -> TraceField {
        return TraceField { key: key.to_ustring(), value: value.to_ustring() };
    }

    pub(crate) fn traced_assertions_msg_field(message: Option<UString>) -> Result<Option<TraceField>, UStringFault> {
        if message.is_none() {
            return Ok(None);
        }
        return Ok(Some(TracedAssertions::traced_assertions_field(UStr::new(&[109,115,103]), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("'")); __s += TestTraceRender::test_trace_render_escape_operand((message).as_deref().unwrap_or(UStr::new(&[])))?.as_ustr(); __s += &(UString::from("'")); __s }).as_str()).as_ustr())));
    }
}
