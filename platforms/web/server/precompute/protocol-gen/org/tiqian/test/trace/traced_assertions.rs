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
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum TracedAssertionsFailFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
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
    pub fn traced_assertions_assert_equals(expected: u32, actual: u32, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_render_int(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_int(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option1) => __option1.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_bool(expected: bool, actual: bool, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_render_bool(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_bool(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option3) => __option3.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_repair_option(expected: Option<RepairOption>, actual: Option<RepairOption>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", match &(expected) { None => "-".to_string(), Some(__option12) => TracedAssertions::traced_assertions_render_repair_option((*__option12).clone()).to_string() }.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", match &(actual) { None => "-".to_string(), Some(__option15) => TracedAssertions::traced_assertions_render_repair_option((*__option15).clone()).to_string() }.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option17) => __option17.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_string(expected: Option<String>, actual: Option<String>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", match &(expected) { None => "-".to_string(), Some(__option26) => TestTraceRender::test_trace_render_render_string(__option26).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_string() }.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", match &(actual) { None => "-".to_string(), Some(__option29) => TestTraceRender::test_trace_render_render_string(__option29).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_string() }.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option31) => __option31.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_string(expected: &str, actual: &str, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_render_string(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_string(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option33) => __option33.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_ic(expected: Ic, actual: Ic, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option35) => __option35.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_enum<T: Clone + std::fmt::Debug + PartialEq>(expected: &T, actual: &T, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", format!("{:?}", expected).as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", format!("{:?}", actual).as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option37) => __option37.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_east_asian_spacing_edges(expected: EastAsianSpacingEdges, actual: EastAsianSpacingEdges, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected.leading != actual.leading || expected.trailing != actual.trailing || expected.contains_wide != actual.contains_wide {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option39) => __option39.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_int_array(expected: &[u32], actual: &[u32], message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TracedAssertions::traced_assertions_render_ints(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TracedAssertions::traced_assertions_render_ints(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option41) => __option41.to_string() }.to_string()), None)?;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if expected[usize::try_from(i).unwrap_or(0)] != actual[usize::try_from(i).unwrap_or(0)] {
                let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option43) => __option43.to_string() }.to_string()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_ints(values: &[u32]) -> Result<String, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("[".encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                buf.extend(", ".encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !format!("{}{}",
            "",
            crate::runtime::int_text::IntText::int_text(values[usize::try_from(i).unwrap_or(0)])
        ).is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            buf.extend(format!("{}{}",
            "",
            crate::runtime::int_text::IntText::int_text(values[usize::try_from(i).unwrap_or(0)])
        ).encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("]".encode_utf16());
        return Ok(String::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_string_array(expected: &[String], actual: &[String], message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_render_string_array(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_string_array(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !TracedAssertions::traced_assertions_same_string_array(&expected, &actual) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option45) => __option45.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_bopomofo_tone(expected: BopomofoTone, actual: BopomofoTone, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.name().to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option47) => __option47.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_font_role(expected: FontRole, actual: Option<FontRole>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", match &(actual) { None => "null".to_string(), Some(__option53) => TracedAssertions::traced_assertions_render_font_role(*__option53).to_string() }.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if match &(actual) { None => true, Some(__option54) => Some(expected) != Some(*__option54) } {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option56) => __option56.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_font_role(value: FontRole) -> String {
        return value.name().to_string();
    }

    pub fn traced_assertions_assert_equals_quote_type(expected: QuoteType, actual: QuoteType, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.name().to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option58) => __option58.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_quote_pair(expected: QuotePair, actual: QuotePair, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected.open_index != actual.open_index || expected.close_index != actual.close_index || expected.quote_type != actual.quote_type {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option60) => __option60.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_quote_pairs(values: &Vec<QuotePair>) -> Result<String, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("[".encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                buf.extend(", ".encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !(values[usize::try_from(i).unwrap_or(0)]).clone().to_string().is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            buf.extend((values[usize::try_from(i).unwrap_or(0)]).clone().to_string().encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("]".encode_utf16());
        return Ok(String::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_quote_pair_array(expected: &Vec<QuotePair>, actual: &Vec<QuotePair>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TracedAssertions::traced_assertions_render_quote_pairs(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TracedAssertions::traced_assertions_render_quote_pairs(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option62) => __option62.to_string() }.to_string()), None)?;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if expected[usize::try_from(i).unwrap_or(0)].open_index != actual[usize::try_from(i).unwrap_or(0)].open_index || expected[usize::try_from(i).unwrap_or(0)].close_index != actual[usize::try_from(i).unwrap_or(0)].close_index ||
expected[usize::try_from(i).unwrap_or(0)].quote_type != actual[usize::try_from(i).unwrap_or(0)].quote_type {
                let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option64) => __option64.to_string() }.to_string()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_int(expected: Option<u32>, actual: Option<u32>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", match &(expected) { None => "-".to_string(), Some(__option73) => TestTraceRender::test_trace_render_render_int(*__option73).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_string() }.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", match &(actual) { None => "-".to_string(), Some(__option76) => TestTraceRender::test_trace_render_render_int(*__option76).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_string() }.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option78) => __option78.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_nullable_float(expected: Option<f64>, actual: Option<f64>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", match &(expected) { None => "-".to_string(), Some(__option87) => TestTraceRender::test_trace_render_render_float(*__option87).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_string() }.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", match &(actual) { None => "-".to_string(), Some(__option90) => TestTraceRender::test_trace_render_render_float(*__option90).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_string() }.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option92) => __option92.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_text_range_array(expected: &Vec<TextRange>, actual: &Vec<TextRange>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let expected_text = TracedAssertions::traced_assertions_render_text_range_array(&expected);
        let actual_text = TracedAssertions::traced_assertions_render_text_range_array(&actual);
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected_text.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual_text.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option94) => __option94.to_string() }.to_string()), None)?;
        }
        for i in 0..match u32::try_from(expected.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if expected[usize::try_from(i).unwrap_or(0)].start != actual[usize::try_from(i).unwrap_or(0)].start || expected[usize::try_from(i).unwrap_or(0)].end != actual[usize::try_from(i).unwrap_or(0)].end {
                let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option96) => __option96.to_string() }.to_string()), None)?;
            }
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_text_range_array(ranges: &Vec<TextRange>) -> String {
        let mut parts: Vec<String> = vec![];
        for i in 0..match u32::try_from(ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((ranges[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return format!("{}{}{}",
            "[",
            { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined[index]); index += 1; } out },
            "]"
        );
    }

    pub fn traced_assertions_assert_equals_int_range(expected: IntRange, actual: IntRange, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TracedAssertions::traced_assertions_render_int_range((expected).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TracedAssertions::traced_assertions_render_int_range((actual).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected.start != actual.start || expected.end != actual.end {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option98) => __option98.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_int_range(r: IntRange) -> Result<String, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("[".encode_utf16());
        let mut i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (i32::from_ne_bytes((r.start).to_ne_bytes())) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                buf.extend(", ".encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_render_int(i)?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            buf.extend(TestTraceRender::test_trace_render_render_int(i)?.encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("]".encode_utf16());
        return Ok(String::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_int_set(expected: SortedSetTable<u32>, actual: SortedSetTable<u32>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TracedAssertions::traced_assertions_render_int_set((expected).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TracedAssertions::traced_assertions_render_int_set((actual).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if u32::from_ne_bytes((expected.size()).to_ne_bytes()) != u32::from_ne_bytes((actual.size()).to_ne_bytes()) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option100) => __option100.to_string() }.to_string()), None)?;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((expected.size()).to_ne_bytes())).to_ne_bytes())) {
            if expected.at(i32::from_ne_bytes((i).to_ne_bytes())) != actual.at(i32::from_ne_bytes((i).to_ne_bytes())) {
                let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option102) => __option102.to_string() }.to_string()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_int_set(values: SortedSetTable<u32>) -> Result<String, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("[".encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((values.size()).to_ne_bytes())).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                buf.extend(", ".encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_render_int(values.at(i32::from_ne_bytes((i).to_ne_bytes())))?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            buf.extend(TestTraceRender::test_trace_render_render_int(values.at(i32::from_ne_bytes((i).to_ne_bytes())))?.encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("]".encode_utf16());
        return Ok(String::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_int_set_unordered(expected: &Vec<u32>, actual: &Vec<u32>, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TracedAssertions::traced_assertions_render_int_list_in_given_order(&expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TracedAssertions::traced_assertions_render_int_list_in_given_order(&actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let mut expected_set: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((expected.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            expected_set.put(&(expected[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        let mut actual_set: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((actual.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            actual_set.put(&(actual[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        let e: SortedSetTable<u32> = expected_set.clone().build();
        let a: SortedSetTable<u32> = actual_set.clone().build();
        if u32::from_ne_bytes((e.size()).to_ne_bytes()) != u32::from_ne_bytes((a.size()).to_ne_bytes()) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option104) => __option104.to_string() }.to_string()), None)?;
        }
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((e.size()).to_ne_bytes())).to_ne_bytes())) {
            if e.at(i32::from_ne_bytes((i).to_ne_bytes())) != a.at(i32::from_ne_bytes((i).to_ne_bytes())) {
                let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option106) => __option106.to_string() }.to_string()), None)?;
            }
            i = u32::wrapping_add(i, 1);
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_int_list_in_given_order(values: &Vec<u32>) -> Result<String, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("[".encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                buf.extend(", ".encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_render_int(values[usize::try_from(i).unwrap_or(0)])?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            buf.extend(TestTraceRender::test_trace_render_render_int(values[usize::try_from(i).unwrap_or(0)])?.encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("]".encode_utf16());
        return Ok(String::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_repair_option_array(expected: &Vec<RepairOption>, actual: &Vec<RepairOption>, message: Option<String>) -> Result<(), TracedAssertionsAssertEqualsRepairOptionArrayFault> {
        let expected_text = TracedAssertions::traced_assertions_render_repair_option_list(&expected).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?;
        let actual_text = TracedAssertions::traced_assertions_render_repair_option_list(&actual).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected_text.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual_text.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertEqualsRepairOptionArrayFault::UStringFaultFault(e))?;
        if expected_text != actual_text {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option108) => __option108.to_string() }.to_string()), None).map_err(|e|
TracedAssertionsAssertEqualsRepairOptionArrayFault::FailFault(e))?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_render_repair_option_list(values: &Vec<RepairOption>) -> Result<String, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("[".encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                buf.extend(", ".encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !TracedAssertions::traced_assertions_render_repair_option((values[usize::try_from(i).unwrap_or(0)]).clone()).is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            buf.extend(TracedAssertions::traced_assertions_render_repair_option((values[usize::try_from(i).unwrap_or(0)]).clone()).encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("]".encode_utf16());
        return Ok(String::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub(crate) fn traced_assertions_render_repair_option(option: RepairOption) -> String {
        return option.to_string();
    }

    pub(crate) fn traced_assertions_render_push_in_allocations(values: &Vec<PushInAllocation>) -> Result<String, UStringFault> {
        let mut buf = Vec::<u16>::new();
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("[".encode_utf16());
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                if let Some(&unit) = buf.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                buf.extend(", ".encode_utf16());
            }
            if let Some(&unit) = buf.last() {
                if unit >= 55296 && unit <= 56319 && !(values[usize::try_from(i).unwrap_or(0)]).clone().to_string().is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            buf.extend((values[usize::try_from(i).unwrap_or(0)]).clone().to_string().encode_utf16());
            i = u32::wrapping_add(i, 1);
        }
        if let Some(&unit) = buf.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        buf.extend("]".encode_utf16());
        return Ok(String::from_utf16(buf.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(buf[buf.len() - 1]) })?);
    }

    pub fn traced_assertions_assert_equals_push_in_allocation_array(expected: &Vec<PushInAllocation>, actual: &Vec<PushInAllocation>, message: Option<String>) -> Result<(), TracedAssertionsAssertEqualsPushInAllocationArrayFault> {
        let expected_text = TracedAssertions::traced_assertions_render_push_in_allocations(&expected).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?;
        let actual_text = TracedAssertions::traced_assertions_render_push_in_allocations(&actual).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected_text.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual_text.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertEqualsPushInAllocationArrayFault::UStringFaultFault(e))?;
        if expected_text != actual_text {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected arrays to be equal.".to_string(), Some(__option110) => __option110.to_string() }.to_string()), None).map_err(|e|
TracedAssertionsAssertEqualsPushInAllocationArrayFault::FailFault(e))?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_bopomofo_reading(expected: BopomofoReading, actual: BopomofoReading, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !TracedAssertions::traced_assertions_same_string_array(&expected.symbols, &actual.symbols) || expected.tone != actual.tone {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option112) => __option112.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_punctuation_class(expected: PunctuationClass, actual: PunctuationClass, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.name().to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option114) => __option114.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_glue_side(expected: GlueSide, actual: GlueSide, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.name().to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option116) => __option116.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_punctuation_glue_placement(expected: PunctuationGluePlacement, actual: PunctuationGluePlacement, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.name().to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option118) => __option118.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_kinsoku_level(expected: KinsokuLevel, actual: KinsokuLevel, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.name().to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option120) => __option120.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_hanging_punctuation_style(expected: HangingPunctuationStyle, actual: HangingPunctuationStyle, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected.name().to_string().as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual.name().to_string().as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option122) => __option122.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_clreq_profile(expected: ClreqProfile, actual: ClreqProfile, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_cap(expected.to_string().as_str()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_cap(actual.to_string().as_str()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !ClreqProfile::clreq_profile_same_profile((expected).clone(), (actual).clone()) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option124) => __option124.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_same_string_array(first: &[String], second: &[String]) -> bool {
        if u32::try_from((first.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((second.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((first.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
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

    pub fn traced_assertions_assert_equals_float(expected: f64, actual: f64, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_render_float(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_float(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option126) => __option126.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_float_tolerance(expected: f64, actual: f64, tolerance: f64, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq-tol", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_render_float(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_float(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"tol", TestTraceRender::test_trace_render_render_float(tolerance).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != expected || actual != actual || ((expected - actual).abs()) > (tolerance) {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal within tolerance.".to_string(), Some(__option128) => __option128.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_rendered(expected: &str, actual: &str, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let expected_text = TestTraceRender::test_trace_render_cap(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let actual_text = TestTraceRender::test_trace_render_cap(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", expected_text.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", actual_text.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected_text != actual_text {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected rendered values to be equal.".to_string(), Some(__option130) => __option130.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_equals_int(expected: u32, actual: u32, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"eq", &vec![
    Some(TracedAssertions::traced_assertions_field(&"expected", TestTraceRender::test_trace_render_render_int(expected).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_int(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if expected != actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected values to be equal.".to_string(), Some(__option132) => __option132.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_true(actual: bool, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"is-true", &vec![
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_bool(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected value to be true.".to_string(), Some(__option134) => __option134.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_false(actual: bool, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"is-false", &vec![
    Some(TracedAssertions::traced_assertions_field(&"actual", TestTraceRender::test_trace_render_render_bool(actual).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if actual {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected value to be false.".to_string(), Some(__option136) => __option136.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_null_rendered(was_null: bool, rendered_actual: &str, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"null", &vec![
    Some(TracedAssertions::traced_assertions_field(&"actual", rendered_actual)),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !was_null {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected value to be null.".to_string(), Some(__option138) => __option138.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_assert_not_null_rendered(was_not_null: bool, rendered_actual: &str, message: Option<String>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"not-null", &vec![
    Some(TracedAssertions::traced_assertions_field(&"actual", rendered_actual)),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        if !was_not_null {
            let _ = TracedAssertions::traced_assertions_fail(Some(match &(message) { None => "Expected value to be non-null.".to_string(), Some(__option140) => __option140.to_string() }.to_string()), None)?;
        }
        Ok(())
    }

    pub fn traced_assertions_record_rendered_not_null(rendered_actual: &str, message: Option<String>) -> Result<(), UStringFault> {
        let _ = TracedAssertions::traced_assertions_record_event(&"not-null", &vec![
    Some(TracedAssertions::traced_assertions_field(&"actual", rendered_actual)),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone())?,
])?;
        Ok(())
    }

    pub fn traced_assertions_assert_fails_with(message: Option<String>, block: Arc<dyn Fn() -> () + Send + Sync>) -> Result<TextRangeError, TracedAssertionsAssertFailsWithFault> {
        let mut caught: Option<TextRangeError> = None;
        let mut derived = false;
        let __outcome: Result<(), TextRangeError> = (|| {
            let __outcome1: Result<(), IllegalStateException> = (|| {
                block();
                Ok(())
            })();
            match __outcome1 {
                Ok(_) => {}
                Err(__caught) => {
                        let error = TextRangeError::Message { text: __caught.message.clone() };
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
                let _ = TracedAssertions::traced_assertions_record_event(&"raises", &vec![
    Some(TracedAssertions::traced_assertions_field(&"exception", if derived { "IllegalStateException".to_string() } else { "TiqianIllegalArgumentException".to_string() }.as_str())),
    Some(TracedAssertions::traced_assertions_field(&"thrown", TestTraceRender::test_trace_render_render_string(format!("{}", __option141).as_str()).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?;
                return Ok((__option141).clone());
            }
            None => {
            }
        }
        let _ = TracedAssertions::traced_assertions_record_event(&"fail", &vec![
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithFault::UStringFaultFault(e))?;
        return Err(TracedAssertionsAssertFailsWithFault::TraceAssertionErrorFault(TraceAssertionError::AssertionFailed { message: match &(message) { None => "Expected an exception.".to_string(), Some(__option142) => __option142.to_string() }.to_string() }));
    }

    pub fn traced_assertions_assert_fails_with_no_such_element(message: Option<String>, block: Arc<dyn Fn() -> () + Send + Sync>) -> Result<NoSuchElementError, TracedAssertionsAssertFailsWithNoSuchElementFault> {
        let __outcome2: Result<(), NoSuchElementError> = (|| {
            block();
            Ok(())
        })();
        match __outcome2 {
            Ok(_) => {}
            Err(error) => {
                let _ = TracedAssertions::traced_assertions_record_event(&"raises", &vec![
    Some(TracedAssertions::traced_assertions_field(&"exception", &"TiqianNoSuchElementException")),
    Some(TracedAssertions::traced_assertions_field(&"thrown", TestTraceRender::test_trace_render_render_string(format!("{}", error).as_str()).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?.as_str())),
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?;
                return Ok(error);
            }
        }
        let _ = TracedAssertions::traced_assertions_record_event(&"fail", &vec![
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsAssertFailsWithNoSuchElementFault::UStringFaultFault(e))?;
        return Err(TracedAssertionsAssertFailsWithNoSuchElementFault::TraceAssertionErrorFault(TraceAssertionError::AssertionFailed { message: match &(message) { None => "Expected an exception.".to_string(), Some(__option143) => __option143.to_string() }.to_string() }));
    }

    pub fn traced_assertions_fail(message: Option<String>, _cause: Option<Exception>) -> Result<(), TracedAssertionsFailFault> {
        let text = match &(message) { None => "Assertion failed.".to_string(), Some(__option144) => __option144.to_string() };
        let _ = TracedAssertions::traced_assertions_record_event(&"fail", &vec![
    TracedAssertions::traced_assertions_msg_field(match &(message) { Some(v) => Some(v.to_string()), None => None }.clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?,
]).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        return Err(TracedAssertionsFailFault::TraceAssertionErrorFault(TraceAssertionError::AssertionFailed { message: text.to_string() }));
    }

    pub fn traced_assertions_assert_does_not_throw<T: Clone>(block: Arc<dyn Fn() -> T + Send + Sync>) -> Result<T, UStringFault> {
        let result = block();
        let _ = TracedAssertions::traced_assertions_record_event(&"no-throw", &vec![])?;
        return Ok((result).clone());
    }

    pub(crate) fn traced_assertions_record_event(name: &str, fields: &Vec<Option<TraceField>>) -> Result<(), UStringFault> {
        if !TestTrace::TEST_TRACE_UPDATE_MODE {
            return Ok(());
        }
        let mut line = Vec::<u16>::new();
        if let Some(&unit) = line.last() {
            if unit >= 55296 && unit <= 56319 && !name.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        line.extend(name.encode_utf16());
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let current = (fields[usize::try_from(index).unwrap_or(0)]).clone();
            match &(current) {
                Some(__option145) => {
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !" ".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    line.extend(" ".encode_utf16());
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !(__option145.key).to_string().is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    line.extend((__option145.key).to_string().encode_utf16());
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !"=".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    line.extend("=".encode_utf16());
                    if let Some(&unit) = line.last() {
                        if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_canonical_numbers((__option145.value).to_string().as_str())?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    line.extend(TestTraceRender::test_trace_render_canonical_numbers((__option145.value).to_string().as_str())?.encode_utf16());
                }
                None => {
                }
            }
            index = u32::wrapping_add(index, 1);
        }
        let rendered = String::from_utf16(line.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(line[line.len() - 1]) })?;
        let recorder = TestTrace::test_trace_current_recorder();
        match &(recorder) {
            Some(__option146) => {
                let _ = __option146.record(rendered.as_str())?;
            }
            None => {
            }
        }
        Ok(())
    }

    pub(crate) fn traced_assertions_field(key: &str, value: &str) -> TraceField {
        return TraceField { key: key.to_string(), value: value.to_string() };
    }

    pub(crate) fn traced_assertions_msg_field(message: Option<String>) -> Result<Option<TraceField>, UStringFault> {
        if message.is_none() {
            return Ok(None);
        }
        return Ok(Some(TracedAssertions::traced_assertions_field(&"msg", format!("{}{}{}",
            "'",
            TestTraceRender::test_trace_render_escape_operand((message).as_deref().unwrap_or(""))?,
            "'"
        ).as_str())));
    }
}
