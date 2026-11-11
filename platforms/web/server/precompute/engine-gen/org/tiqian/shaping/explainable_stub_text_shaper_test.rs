#![cfg(test)]

use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::shaping::explainable_stub_text_shaper_test_support::ExplainableStubTextShaperTestSupport;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestShapesSingleCjkClusterWithOneEmAdvanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestShapesClreqDashSubstitutionAsTwoEmDisplayClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ExplainableStubTextShaperTestKeepsLatinRunAsSingleShapedClusterWithNominalGlyphsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[69,120,112,108,97,105,110,97,98,108,101,83,116,117,98,84,101,120,116,83,104,97,112,101,114,84,101,115,116]))).section(UStr::new(&[115,104,97,112,101,115,83,105,110,103,108,101,67,106,107,67,108,117,115,116,101,114,87,105,116,104,79,110,101,69,109,65,100,118,97,110,99,101]));
        let result = Arc::new(Mutex::new(ExplainableStubTextShaper::new())).lock().unwrap().shape(ExplainableStubTextShaperTestSupport::explainable_stub_text_shaper_test_support_input(UStr::new(&[20013]), FontRole::CjkText, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[20013]), ((result.clusters[0usize]).clone().text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[20013]), ((result.clusters[0usize]).clone().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, result.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((result.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[83,116,117,98]), ((result.decisions[0usize]).clone().source).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn keeps_latin_run_as_single_shaped_cluster_with_nominal_glyphs() {
    testlib::run("org.tiqian.shaping.ExplainableStubTextShaperTest.keepsLatinRunAsSingleShapedClusterWithNominalGlyphs", "org.tiqian.shaping.ExplainableStubTextShaperTest.keepsLatinRunAsSingleShapedClusterWithNominalGlyphs", || {
        TestTraceRecorder::new(&(UStr::new(&[69,120,112,108,97,105,110,97,98,108,101,83,116,117,98,84,101,120,116,83,104,97,112,101,114,84,101,115,116]))).section(UStr::new(&[107,101,101,112,115,76,97,116,105,110,82,117,110,65,115,83,105,110,103,108,101,83,104,97,112,101,100,67,108,117,115,116,101,114,87,105,116,104,78,111,109,105,110,97,108,71,108,121,112,104,115]));
        let result = Arc::new(Mutex::new(ExplainableStubTextShaper::new())).lock().unwrap().shape(ExplainableStubTextShaperTestSupport::explainable_stub_text_shaper_test_support_input(UStr::new(&[72,101,108,108,111]), FontRole::LatinText, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[72,101,108,108,111]), ((result.clusters[0usize]).clone().text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(80.0f64, result.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, u32::try_from(((result.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, result.decisions[0usize].glyph_count, None).unwrap();
    });
}

#[test]
fn shapes_clreq_dash_substitution_as_two_em_display_cluster() {
    testlib::run("org.tiqian.shaping.ExplainableStubTextShaperTest.shapesClreqDashSubstitutionAsTwoEmDisplayCluster", "org.tiqian.shaping.ExplainableStubTextShaperTest.shapesClreqDashSubstitutionAsTwoEmDisplayCluster", || {
        TestTraceRecorder::new(&(UStr::new(&[69,120,112,108,97,105,110,97,98,108,101,83,116,117,98,84,101,120,116,83,104,97,112,101,114,84,101,115,116]))).section(UStr::new(&[115,104,97,112,101,115,67,108,114,101,113,68,97,115,104,83,117,98,115,116,105,116,117,116,105,111,110,65,115,84,119,111,69,109,68,105,115,112,108,97,121,67,108,117,115,116,101,114]));
        let result = Arc::new(Mutex::new(ExplainableStubTextShaper::new())).lock().unwrap().shape(ExplainableStubTextShaperTestSupport::explainable_stub_text_shaper_test_support_input(UStr::new(&[8212,8212]), FontRole::CjkPunctuation, Some(UString::from("⸺"))).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), ((result.clusters[0usize]).clone().text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[11834]), ((result.clusters[0usize]).clone().display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, result.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((result.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, (result.glyph_runs[0usize]).clone().glyphs[0usize].advance, None).unwrap();
    });
}
