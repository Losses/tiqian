#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphFns;
use crate::org::tiqian::layout::prepared_paragraph_plan_construction_test_support::PreparedParagraphPlanConstructionTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestStyleDeltaListsOnlyPaintFieldsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestOpenTypeFeaturesAndRenderFontFamilyAttachPerClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestMultiUnitClusterMarksShapingBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestInlineObjectCellEmitsAdvanceOverrideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    SupportRunEvidenceFault(crate::org::tiqian::layout::prepared_paragraph_plan_construction_test_support::PreparedParagraphPlanConstructionTestSupportRunEvidenceFault),
}

impl From<PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault> for crate::org::tiqian::layout::prepared_paragraph_plan_construction_test_support::PreparedParagraphPlanConstructionTestSupportRunEvidenceFault {
    fn from(value: PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::SupportRunEvidenceFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph_plan_construction_test_support::PreparedParagraphPlanConstructionTestSupportRunEvidenceFault> for PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph_plan_construction_test_support::PreparedParagraphPlanConstructionTestSupportRunEvidenceFault) -> Self {
        PreparedParagraphPlanConstructionTestDashClusterEmitsShapingEvidenceBlockFault::SupportRunEvidenceFault(value)
    }
}

#[test]
fn open_type_features_and_render_font_family_attach_per_cluster() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.openTypeFeaturesAndRenderFontFamilyAttachPerCluster", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.openTypeFeaturesAndRenderFontFamilyAttachPerCluster", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphPlanConstructionTest");
        t.section(&"openTypeFeaturesAndRenderFontFamilyAttachPerCluster");
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_open_type_features_and_render_font_family_attach_per_cluster().unwrap(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"openTypeFeatures\":[\"kern\",\"liga\"]", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"renderFontFamily\":\"Noto Serif CJK\"", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&json, "shapingBoundary", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
    });
}

#[test]
fn multi_unit_cluster_marks_shaping_boundary() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.multiUnitClusterMarksShapingBoundary", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.multiUnitClusterMarksShapingBoundary", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphPlanConstructionTest");
        t.section(&"multiUnitClusterMarksShapingBoundary");
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_multi_unit_cluster_marks_shaping_boundary().unwrap(), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"shapingBoundary\":true", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
    });
}

#[test]
fn inline_object_cell_emits_advance_override() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.inlineObjectCellEmitsAdvanceOverride", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.inlineObjectCellEmitsAdvanceOverride", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphPlanConstructionTest");
        t.section(&"inlineObjectCellEmitsAdvanceOverride");
        let r = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_inline_object_cell_emits_advance_override().unwrap();
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"inlineObject\":24", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"advance\":10", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
        let mut empty_clusters: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            empty_clusters.push(Cluster::new((c.range).clone(), (c.text).to_string().as_str(), (c.font_key).to_string().as_str(), c.advance, if c.range.clone().start == 1 { Some("".to_string()) } else { Some((c.display_text).to_string()) }.clone(), Some(0.0), Some(0.0),
Some(0.0)));
        }
        let empty_display = LayoutResult::new((r.input).clone(), (r.size).clone(), (empty_clusters).clone(), (r.glyph_runs).clone(), (r.lines).clone(), (r.debug).clone());
        let plain = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((empty_display).clone(), false).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&plain, "\"inlineObject\"", 0)).to_ne_bytes())) <= 2147483647, Some((plain).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&plain, "\"rangeStart\":1", 0)).to_ne_bytes())) <= 2147483647, Some((plain).to_string())).unwrap();
        let evidence = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((empty_display).clone(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&evidence, "\"inlineObject\":24", 0)).to_ne_bytes())) <= 2147483647, Some((evidence).to_string())).unwrap();
    });
}

#[test]
fn style_delta_lists_only_paint_fields() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.styleDeltaListsOnlyPaintFields", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.styleDeltaListsOnlyPaintFields", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphPlanConstructionTest");
        t.section(&"styleDeltaListsOnlyPaintFields");
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_style_delta_lists_only_paint_fields().unwrap(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"style\":{\"fontSize\":20,\"fontWeight\":700,\"italic\":true}", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"style\":{}", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::wrapping_sub(u32::try_from((u_string::split(&json, &"\"style\":").len()) & 0xFFFF_FFFF).unwrap_or(0), 1), None).unwrap();
    });
}

#[test]
fn dash_cluster_emits_shaping_evidence_block() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.dashClusterEmitsShapingEvidenceBlock", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.dashClusterEmitsShapingEvidenceBlock", || {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_run_evidence(&"dashClusterEmitsShapingEvidenceBlock",
PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_dash_cluster_emits_shaping_evidence_block().unwrap()).unwrap();
    });
}

#[test]
fn punctuation_ink_floor_and_latin_role_mark_cells() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.punctuationInkFloorAndLatinRoleMarkCells", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.punctuationInkFloorAndLatinRoleMarkCells", || {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_run_punctuation().unwrap();
    });
}

#[test]
fn zero_width_break_cluster_survives_empty_display_text() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.zeroWidthBreakClusterSurvivesEmptyDisplayText", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.zeroWidthBreakClusterSurvivesEmptyDisplayText", || {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_run_zero_width().unwrap();
    });
}

#[test]
fn paragraph_evidence_emits_every_section() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.paragraphEvidenceEmitsEverySection", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.paragraphEvidenceEmitsEverySection", || {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_run_paragraph_evidence().unwrap();
    });
}

#[test]
fn negative_zero_and_exponent_widths_normalize() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.negativeZeroAndExponentWidthsNormalize", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.negativeZeroAndExponentWidthsNormalize", || {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_run_negative_zero().unwrap();
    });
}

#[test]
fn json_string_escapes_quotes_backslashes_and_control_characters() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.jsonStringEscapesQuotesBackslashesAndControlCharacters", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.jsonStringEscapesQuotesBackslashesAndControlCharacters", || {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_run_escapes().unwrap();
    });
}

#[test]
fn plan_with_diagnostics_lists_capability_issues_and_advance_suspects() {
    testlib::run("org.tiqian.layout.PreparedParagraphPlanConstructionTest.planWithDiagnosticsListsCapabilityIssuesAndAdvanceSuspects", "org.tiqian.layout.PreparedParagraphPlanConstructionTest.planWithDiagnosticsListsCapabilityIssuesAndAdvanceSuspects", || {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_run_diagnostics().unwrap();
    });
}
