#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::shaping::replayable_font_backend::FontBackendCapabilityIssue;
use crate::org::tiqian::shaping::replayable_font_backend::FontBackendCapabilityReport;
use crate::org::tiqian::shaping::replayable_font_backend::FontFaceIdImpl;
use crate::org::tiqian::shaping::replayable_font_backend::ReplayableFontCatalog;
use crate::org::tiqian::shaping::replayable_font_backend::ReplayableFontFaceDescriptor;
use crate::org::tiqian::shaping::replayable_font_backend::ReplayableFontFaceRequest;
use crate::org::tiqian::shaping::replayable_font_backend_coverage_test_support::ReplayableFontBackendCoverageTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ReplayableFontBackendCoverageTestFaceRequestRejectsNonPositiveAndNonFiniteFontSizeFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ReplayableFontBackendCoverageTestFaceDescriptorDefaultsAreStableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ReplayableFontBackendCoverageTestCatalogContractResolvesByRequestFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault) -> Self {
        match value {
            ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ReplayableFontBackendCoverageTestCapabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn font_face_id_rejects_blank_and_keeps_value() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.fontFaceIdRejectsBlankAndKeepsValue", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.fontFaceIdRejectsBlankAndKeepsValue", || {
        TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,97,98,108,101,70,111,110,116,66,97,99,107,101,110,100,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[102,111,110,116,70,97,99,101,73,100,82,101,106,101,99,116,115,66,108,97,110,107,65,110,100,75,101,101,112,115,86,97,108,117,101]));
        let id = FontFaceIdImpl::font_face_id_impl_of(UStr::new(&[110,111,116,111,45,99,106,107,45,49])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[110,111,116,111,45,99,106,107,45,49]), id.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[110,111,116,111,45,99,106,107,45,49]), FontFaceIdImpl::font_face_id_impl_to_string(id.as_ustr()).as_ustr(), None).unwrap();
        let blank = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        FontFaceIdImpl::font_face_id_impl_of(UStr::new(&[32])).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", format!("{}", blank)).as_str()), UString::from("blank").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some(UString::from(format!("{}", format!("{}", blank)).as_str()))).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        FontFaceIdImpl::font_face_id_impl_of(UStr::new(&[])).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn face_descriptor_defaults_are_stable() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceDescriptorDefaultsAreStable", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceDescriptorDefaultsAreStable", || {
        TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,97,98,108,101,70,111,110,116,66,97,99,107,101,110,100,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[102,97,99,101,68,101,115,99,114,105,112,116,111,114,68,101,102,97,117,108,116,115,65,114,101,83,116,97,98,108,101]));
        let descriptor = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(UStr::new(&[102,97,99,101,45,97])).unwrap().as_ustr(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec![UString::from("Serif").to_ustring()]), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::CjkText]).to_vec(), &(UStr::new(&[98,117,110,100,108,101,100,47,110,111,116,111,46,116,116,102])), Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr()))).build())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(400, descriptor.weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(descriptor.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, descriptor.collection_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((descriptor.variation_axes.size()) as u32).to_ne_bytes()) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[102,97,99,101,45,97]), FontFaceIdImpl::font_face_id_impl_to_string(descriptor.id.as_ustr()).as_ustr(), None).unwrap();
        let varied = ReplayableFontFaceDescriptor::new(descriptor.id.as_ustr(), (descriptor.family_aliases).clone(), descriptor.roles.to_vec(), (descriptor.source_label).to_ustring().as_ustr(), Some(700), Some(true), Some(2), Some(ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_axes(UStr::new(&[119,103,104,116]), 700.0f64))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(700, varied.weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(varied.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, varied.collection_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(700.0f64, varied.variation_axes.get(&(UString::from("wght")).to_ustring()).unwrap(), None).unwrap();
    });
}

#[test]
fn face_request_rejects_non_positive_and_non_finite_font_size() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceRequestRejectsNonPositiveAndNonFiniteFontSize", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceRequestRejectsNonPositiveAndNonFiniteFontSize", || {
        TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,97,98,108,101,70,111,110,116,66,97,99,107,101,110,100,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[102,97,99,101,82,101,113,117,101,115,116,82,101,106,101,99,116,115,78,111,110,80,111,115,105,116,105,118,101,65,110,100,78,111,110,70,105,110,105,116,101,70,111,110,116,83,105,122,101]));
        let request = ReplayableFontFaceRequest::new(FontRole::LatinText, vec![UString::from("Plex").to_ustring()].to_vec(), 15.0f64, 400u32, false, &(UStr::new(&[122,104,45,67,78])), &(UStr::new(&[65]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(request.role), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(15.0f64, request.font_size, None).unwrap();
        let error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), 0.0f64, 400u32, false, &(UStr::new(&[])), &(UStr::new(&[65]))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", format!("{}", error)).as_str()), UString::from("positive and finite").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some(UString::from(format!("{}", format!("{}", error)).as_str()))).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), -1.0f64, 400u32, false, &(UStr::new(&[])), &(UStr::new(&[65]))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), f64::NAN, 400u32, false, &(UStr::new(&[])), &(UStr::new(&[65]))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), f64::INFINITY, 400u32, false, &(UStr::new(&[])), &(UStr::new(&[65]))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn capability_report_replay_flag_requires_faces_and_no_missing_face_issue() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.capabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssue", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.capabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssue", || {
        TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,97,98,108,101,70,111,110,116,66,97,99,107,101,110,100,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[99,97,112,97,98,105,108,105,116,121,82,101,112,111,114,116,82,101,112,108,97,121,70,108,97,103,82,101,113,117,105,114,101,115,70,97,99,101,115,65,110,100,78,111,77,105,115,115,105,110,103,70,97,99,101,73,115,115,117,101]));
        let face = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(UStr::new(&[102,97,99,101,45,97])).unwrap().as_ustr(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec![UString::from("Serif").to_ustring()]), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::CjkText]).to_vec(), &(UStr::new(&[98,121,116,101,115])), Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr()))).build())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontBackendCapabilityReport::new(&(UStr::new(&[98])), &(UStr::new(&[107])), vec![].to_vec(), Some(vec![])).unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontBackendCapabilityReport::new(&(UStr::new(&[98])), &(UStr::new(&[107])), vec![(face).clone()].to_vec(), Some(vec![
    (FontBackendCapabilityIssue::new(&(UStr::new(&[77,105,115,115,105,110,103,67,111,110,116,114,111,108,108,101,100,70,111,110,116,70,97,99,101])), &(UStr::new(&[103,111,110,101]))).unwrap()).clone(),
])).unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(FontBackendCapabilityReport::new(&(UStr::new(&[98])), &(UStr::new(&[107])), vec![(face).clone()].to_vec(), Some(vec![
    (FontBackendCapabilityIssue::new(&(UStr::new(&[79,116,104,101,114])), &(UStr::new(&[110,111,116,101]))).unwrap()).clone(),
])).unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(FontBackendCapabilityReport::new(&(UStr::new(&[98])), &(UStr::new(&[107])), vec![(face).clone()].to_vec(), Some(vec![])).unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
    });
}

#[test]
fn catalog_contract_resolves_by_request() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.catalogContractResolvesByRequest", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.catalogContractResolvesByRequest", || {
        TestTraceRecorder::new(&(UStr::new(&[82,101,112,108,97,121,97,98,108,101,70,111,110,116,66,97,99,107,101,110,100,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[99,97,116,97,108,111,103,67,111,110,116,114,97,99,116,82,101,115,111,108,118,101,115,66,121,82,101,113,117,101,115,116]));
        let cjk_face = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(UStr::new(&[102,97,99,101,45,99,106,107])).unwrap().as_ustr(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec![UString::from("Noto Serif CJK").to_ustring()]), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::CjkText]).to_vec(), &(UStr::new(&[98,121,116,101,115])), Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr()))).build())).unwrap();
        let latin_face = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(UStr::new(&[102,97,99,101,45,108,97,116,105,110])).unwrap().as_ustr(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec![UString::from("Plex").to_ustring()]), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::LatinText]).to_vec(), &(UStr::new(&[98,121,116,101,115])), Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr()))).build())).unwrap();
        let catalog: Box<dyn ReplayableFontCatalog> = Box::new(CatalogImpl::new(vec![(cjk_face).clone(), (latin_face).clone()].to_vec()));
        let _ = TracedAssertions::traced_assertions_assert_true(catalog.get_capability_report().unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
        let hit = catalog.resolve(ReplayableFontFaceRequest::new(FontRole::LatinText, vec![UString::from("Plex").to_ustring()].to_vec(), 12.0f64, 400u32, false, &(UStr::new(&[122,104,45,67,78])), &(UStr::new(&[65]))).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[102,97,99,101,45,108,97,116,105,110]), match &(hit) { None => UString::from("-"), Some(__option2) => FontFaceIdImpl::font_face_id_impl_to_string(__option2.id.as_ustr()).to_ustring() }.as_ustr(), None).unwrap();
        let miss = catalog.resolve(ReplayableFontFaceRequest::new(FontRole::LatinText, vec![UString::from("Missing").to_ustring()].to_vec(), 12.0f64, 400u32, false, &(UStr::new(&[122,104,45,67,78])), &(UStr::new(&[65]))).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(miss.is_none(), UStr::new(&[45]), None).unwrap();
    });
}

#[derive(Clone)]
pub struct CatalogImpl {
    pub(crate) stored: Vec<ReplayableFontFaceDescriptor>,
}

impl CatalogImpl {
    pub fn new(faces: Vec<ReplayableFontFaceDescriptor>) -> Self {
        Self {
            stored: faces,
        }
    }

    pub fn get_faces(&self) -> Vec<ReplayableFontFaceDescriptor> {
        return ((self.stored).clone()).clone();
    }

    pub fn get_capability_report(&self) -> Result<FontBackendCapabilityReport, TextRangeError> {
        return Ok(FontBackendCapabilityReport::new(&(UStr::new(&[116,101,115,116])), &(UStr::new(&[98,121,116,101,115])), (self.stored).clone().to_vec(), Some(vec![]))?);
    }

    pub fn resolve(&self, request: ReplayableFontFaceRequest) -> Option<ReplayableFontFaceDescriptor> {
        {
            let _g1 = (self.stored).clone();
            for face in &_g1 {
                if u32::from_ne_bytes(((match (face.roles).clone().iter().position(|e| e == &request.role) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) <= 2147483647 {
                    let mut _g = 0u32;
                    let _g1 = request.preferred_families.clone();
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let family = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if face.family_aliases.clone().has(&(family).to_ustring()) {
                            return Some((*face).clone());
                        }
                    }
                }
            }
        }
        return None;
    }
}

impl ReplayableFontCatalog for CatalogImpl {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.shaping.ReplayableFontBackendCoverageTest.CatalogImpl"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ReplayableFontCatalog> {
        Box::new(self.clone())
    }

    fn get_faces(&self) -> Vec<ReplayableFontFaceDescriptor> {
        return ((self.stored).clone()).clone();
    }

    fn get_capability_report(&self) -> Result<FontBackendCapabilityReport, TextRangeError> {
        return Ok(FontBackendCapabilityReport::new(&(UStr::new(&[116,101,115,116])), &(UStr::new(&[98,121,116,101,115])), (self.stored).clone().to_vec(), Some(vec![]))?);
    }

    fn resolve(&self, request: ReplayableFontFaceRequest) -> Option<ReplayableFontFaceDescriptor> {
        {
            let _g1 = (self.stored).clone();
            for face in &_g1 {
                if u32::from_ne_bytes(((match (face.roles).clone().iter().position(|e| e == &request.role) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) <= 2147483647 {
                    let _g1 = request.preferred_families.clone();
                    for family in &_g1 {
                        if face.family_aliases.clone().has(&(family).to_ustring()) {
                            return Some((*face).clone());
                        }
                    }
                }
            }
        }
        return None;
    }
}
