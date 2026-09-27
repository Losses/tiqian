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
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ReplayableFontBackendCoverageTestFontFaceIdRejectsBlankAndKeepsValueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
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
        TestTraceRecorder::new("ReplayableFontBackendCoverageTest").section(&"fontFaceIdRejectsBlankAndKeepsValue");
        let id = FontFaceIdImpl::font_face_id_impl_of(&"noto-cjk-1").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"noto-cjk-1", id.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"noto-cjk-1", FontFaceIdImpl::font_face_id_impl_to_string(id.as_str()).as_str(), None).unwrap();
        let blank = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        FontFaceIdImpl::font_face_id_impl_of(&" ").map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", blank), "blank", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}", blank)).to_string())).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        FontFaceIdImpl::font_face_id_impl_of(&"").map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn face_descriptor_defaults_are_stable() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceDescriptorDefaultsAreStable", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceDescriptorDefaultsAreStable", || {
        TestTraceRecorder::new("ReplayableFontBackendCoverageTest").section(&"faceDescriptorDefaultsAreStable");
        let descriptor = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(&"face-a").unwrap().as_str(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec!["Serif".to_string()]),
ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::CjkText]).to_vec(), "bundled/noto.ttf", Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b|
SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str()))).build())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(400, descriptor.weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(descriptor.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, descriptor.collection_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((descriptor.variation_axes.size()).to_ne_bytes()) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"face-a", FontFaceIdImpl::font_face_id_impl_to_string(descriptor.id.as_str()).as_str(), None).unwrap();
        let varied = ReplayableFontFaceDescriptor::new(descriptor.id.as_str(), (descriptor.family_aliases).clone(), descriptor.roles.to_vec(), (descriptor.source_label).to_string().as_str(), Some(700), Some(true), Some(2),
Some(ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_axes(&"wght", 700.0f64))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(700, varied.weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(varied.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, varied.collection_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(700.0f64, varied.variation_axes.get(&("wght").to_string()).unwrap(), None).unwrap();
    });
}

#[test]
fn face_request_rejects_non_positive_and_non_finite_font_size() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceRequestRejectsNonPositiveAndNonFiniteFontSize", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.faceRequestRejectsNonPositiveAndNonFiniteFontSize", || {
        TestTraceRecorder::new("ReplayableFontBackendCoverageTest").section(&"faceRequestRejectsNonPositiveAndNonFiniteFontSize");
        let request = ReplayableFontFaceRequest::new(FontRole::LatinText, vec!["Plex".to_string()].to_vec(), 15.0f64, 400u32, false, "zh-CN", "A").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(request.role), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(15.0f64, request.font_size, None).unwrap();
        let error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), 0.0f64, 400u32, false, "", "A").map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", error), "positive and finite", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}", error)).to_string())).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), -1.0f64, 400u32, false, "", "A").map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), f64::NAN, 400u32, false, "", "A").map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ReplayableFontFaceRequest::new(FontRole::LatinText, vec![].to_vec(), f64::INFINITY, 400u32, false, "", "A").map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn capability_report_replay_flag_requires_faces_and_no_missing_face_issue() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.capabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssue", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.capabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssue", || {
        TestTraceRecorder::new("ReplayableFontBackendCoverageTest").section(&"capabilityReportReplayFlagRequiresFacesAndNoMissingFaceIssue");
        let face = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(&"face-a").unwrap().as_str(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec!["Serif".to_string()]),
ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::CjkText]).to_vec(), "bytes", Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b|
SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str()))).build())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontBackendCapabilityReport::new("b", "k", vec![].to_vec(), Some(vec![])).unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontBackendCapabilityReport::new("b", "k", vec![(face).clone()].to_vec(), Some(vec![(FontBackendCapabilityIssue::new("MissingControlledFontFace",
"gone").unwrap()).clone()])).unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(FontBackendCapabilityReport::new("b", "k", vec![(face).clone()].to_vec(), Some(vec![(FontBackendCapabilityIssue::new("Other", "note").unwrap()).clone()])).unwrap().get_can_replay_from_controlled_bytes(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(FontBackendCapabilityReport::new("b", "k", vec![(face).clone()].to_vec(), Some(vec![])).unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
    });
}

#[test]
fn catalog_contract_resolves_by_request() {
    testlib::run("org.tiqian.shaping.ReplayableFontBackendCoverageTest.catalogContractResolvesByRequest", "org.tiqian.shaping.ReplayableFontBackendCoverageTest.catalogContractResolvesByRequest", || {
        TestTraceRecorder::new("ReplayableFontBackendCoverageTest").section(&"catalogContractResolvesByRequest");
        let cjk_face = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(&"face-cjk").unwrap().as_str(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec!["Noto Serif CJK".to_string()]),
ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::CjkText]).to_vec(), "bytes", Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b|
SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str()))).build())).unwrap();
        let latin_face = ReplayableFontFaceDescriptor::new(FontFaceIdImpl::font_face_id_impl_of(&"face-latin").unwrap().as_str(), ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_strings(&vec!["Plex".to_string()]),
ReplayableFontBackendCoverageTestSupport::replayable_font_backend_coverage_test_support_roles(&vec![FontRole::LatinText]).to_vec(), "bytes", Some(400), Some(false), Some(0), Some(SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b|
SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str()))).build())).unwrap();
        let catalog: Box<dyn ReplayableFontCatalog> = Box::new(CatalogImpl::new(vec![(cjk_face).clone(), (latin_face).clone()].to_vec()));
        let _ = TracedAssertions::traced_assertions_assert_true(catalog.get_capability_report().unwrap().get_can_replay_from_controlled_bytes(), None).unwrap();
        let hit = catalog.resolve(ReplayableFontFaceRequest::new(FontRole::LatinText, vec!["Plex".to_string()].to_vec(), 12.0f64, 400u32, false, "zh-CN", "A").unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"face-latin", match &(hit) { None => "-".to_string(), Some(__option2) => FontFaceIdImpl::font_face_id_impl_to_string(__option2.id.as_str()).to_string() }.as_str(), None).unwrap();
        let miss = catalog.resolve(ReplayableFontFaceRequest::new(FontRole::LatinText, vec!["Missing".to_string()].to_vec(), 12.0f64, 400u32, false, "zh-CN", "A").unwrap());
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(miss.is_none(), &"-", None).unwrap();
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
        return Ok(FontBackendCapabilityReport::new("test", "bytes", (self.stored).clone().to_vec(), Some(vec![]))?);
    }

    pub fn resolve(&self, request: ReplayableFontFaceRequest) -> Option<ReplayableFontFaceDescriptor> {
        {
            let _g1 = (self.stored).clone();
            for face in &_g1 {
                if u32::from_ne_bytes((match (face.roles).clone().iter().position(|e| e == &request.role) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) <= 2147483647 {
                    let mut _g = 0u32;
                    let _g1 = request.preferred_families.clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let family = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if face.family_aliases.clone().has(&(family).to_string()) {
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
        return Ok(FontBackendCapabilityReport::new("test", "bytes", (self.stored).clone().to_vec(), Some(vec![]))?);
    }

    fn resolve(&self, request: ReplayableFontFaceRequest) -> Option<ReplayableFontFaceDescriptor> {
        {
            let _g1 = (self.stored).clone();
            for face in &_g1 {
                if u32::from_ne_bytes((match (face.roles).clone().iter().position(|e| e == &request.role) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) <= 2147483647 {
                    let _g1 = request.preferred_families.clone();
                    for family in &_g1 {
                        if face.family_aliases.clone().has(&(family).to_string()) {
                            return Some((*face).clone());
                        }
                    }
                }
            }
        }
        return None;
    }
}
