#![cfg(test)]

use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::shaping::explainable_stub_text_shaper_test_support::ExplainableStubTextShaperTestSupport;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault) -> Self {
        match value {
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault) -> Self {
        match value {
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault) -> Self {
        match value {
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault) -> Self {
        match value {
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault) -> Self {
        match value {
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn shapes_single_cjk_cluster_with_one_em_advance() {
    testlib::run("org.tiqian.shaping.ExplainableStubTextShaperTest.shapesSingleCjkClusterWithOneEmAdvance", "org.tiqian.shaping.ExplainableStubTextShaperTest.shapesSingleCjkClusterWithOneEmAdvance", || {
        TestTraceRecorder::new("ExplainableStubTextShaperTest").section(&"shapesSingleCjkClusterWithOneEmAdvance");
        let result = ExplainableStubTextShaper::new().shape(ExplainableStubTextShaperTestSupport::explainable_stub_text_shaper_test_support_input(&"中", FontRole::CjkText, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"中", ((result.clusters[0usize]).clone().text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"中", ((result.clusters[0usize]).clone().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, result.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((result.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Stub", ((result.decisions[0usize]).clone().source).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn keeps_latin_run_as_single_shaped_cluster_with_nominal_glyphs() {
    testlib::run("org.tiqian.shaping.ExplainableStubTextShaperTest.keepsLatinRunAsSingleShapedClusterWithNominalGlyphs", "org.tiqian.shaping.ExplainableStubTextShaperTest.keepsLatinRunAsSingleShapedClusterWithNominalGlyphs", || {
        TestTraceRecorder::new("ExplainableStubTextShaperTest").section(&"keepsLatinRunAsSingleShapedClusterWithNominalGlyphs");
        let result = ExplainableStubTextShaper::new().shape(ExplainableStubTextShaperTestSupport::explainable_stub_text_shaper_test_support_input(&"Hello", FontRole::LatinText, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Hello", ((result.clusters[0usize]).clone().text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(80.0f64, result.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, u32::try_from(((result.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, result.decisions[0usize].glyph_count, None).unwrap();
    });
}

#[test]
fn shapes_clreq_dash_substitution_as_two_em_display_cluster() {
    testlib::run("org.tiqian.shaping.ExplainableStubTextShaperTest.shapesClreqDashSubstitutionAsTwoEmDisplayCluster", "org.tiqian.shaping.ExplainableStubTextShaperTest.shapesClreqDashSubstitutionAsTwoEmDisplayCluster", || {
        TestTraceRecorder::new("ExplainableStubTextShaperTest").section(&"shapesClreqDashSubstitutionAsTwoEmDisplayCluster");
        let result = ExplainableStubTextShaper::new().shape(ExplainableStubTextShaperTestSupport::explainable_stub_text_shaper_test_support_input(&"——", FontRole::CjkPunctuation, Some("⸺".to_string())).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", ((result.clusters[0usize]).clone().text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"⸺", ((result.clusters[0usize]).clone().display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, result.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((result.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, (result.glyph_runs[0usize]).clone().glyphs[0usize].advance, None).unwrap();
    });
}
