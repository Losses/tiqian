#![cfg(test)]

use crate::org::tiqian::core::source_boundary_bias::SourceBoundaryBias;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        SourceInteractionBoundariesCoverageTestCoercionHonoursEveryBiasAndEdgeCaseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceInteractionBoundariesCoverageTestAssertBoundariesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for SourceInteractionBoundariesCoverageTestAssertBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceInteractionBoundariesCoverageTestAssertBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestAssertBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: SourceInteractionBoundariesCoverageTestAssertBoundariesFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestAssertBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestAssertBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: SourceInteractionBoundariesCoverageTestAssertBoundariesFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<SourceInteractionBoundariesCoverageTestAssertBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: SourceInteractionBoundariesCoverageTestAssertBoundariesFault) -> Self {
        match value {
            SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for SourceInteractionBoundariesCoverageTestAssertBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        SourceInteractionBoundariesCoverageTestAssertBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for SourceInteractionBoundariesCoverageTestAssertBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for SourceInteractionBoundariesCoverageTestAssertBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn crlf_stays_one_unit() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.crlfStaysOneUnit", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.crlfStaysOneUnit", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[99,114,108,102,83,116,97,121,115,79,110,101,85,110,105,116]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[13,10])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,93]), UStr::new(&[13])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), UStr::new(&[97,10])).unwrap();
    });
}

#[test]
fn regional_indicators_pair_up() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.regionalIndicatorsPairUp", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.regionalIndicatorsPairUp", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[114,101,103,105,111,110,97,108,73,110,100,105,99,97,116,111,114,115,80,97,105,114,85,112]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,52,93]), TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806, 55356, 56808]).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,52,44,32,54,93]), TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806, 55356, 56806, 55356, 56806]).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,44,32,51,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806]).as_ustr(); __s += &(UString::from("A")); __s }).as_str()).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806]).as_ustr()).unwrap();
    });
}

#[test]
fn hangul_jamo_runs_merge_into_syllable_blocks() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.hangulJamoRunsMergeIntoSyllableBlocks", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.hangulJamoRunsMergeIntoSyllableBlocks", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[104,97,110,103,117,108,74,97,109,111,82,117,110,115,77,101,114,103,101,73,110,116,111,83,121,108,108,97,98,108,101,66,108,111,99,107,115]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,51,93]), UStr::new(&[4352,4352,4449])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,51,93]), UStr::new(&[4352,4449,4520])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,52,93]), UStr::new(&[4352,4449,4520,4520])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), UStr::new(&[4352,65])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,44,32,51,93]), UStr::new(&[4352,4449,65])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[43360,4449])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[4352,55216])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,51,93]), UStr::new(&[4352,4449,55243])).unwrap();
    });
}

#[test]
fn precomposed_hangul_syllables_absorb_jamo() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.precomposedHangulSyllablesAbsorbJamo", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.precomposedHangulSyllablesAbsorbJamo", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[112,114,101,99,111,109,112,111,115,101,100,72,97,110,103,117,108,83,121,108,108,97,98,108,101,115,65,98,115,111,114,98,74,97,109,111]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,51,93]), UStr::new(&[44032,4449,4520])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[44033,4520])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), UStr::new(&[44033,65])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[44032,4520])).unwrap();
    });
}

#[test]
fn extenders_attach_to_the_preceding_unit() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.extendersAttachToThePrecedingUnit", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.extendersAttachToThePrecedingUnit", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[101,120,116,101,110,100,101,114,115,65,116,116,97,99,104,84,111,84,104,101,80,114,101,99,101,100,105,110,103,85,110,105,116]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[97,769])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[97,65039])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,51,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("a")); __s += TestHelpers::test_helpers_surrogate_text(&vec![56128, 56576]).as_ustr(); __s }).as_str()).as_ustr()).unwrap();
        let scotland = TestHelpers::test_helpers_surrogate_text(&vec![55356, 57332, 56128, 56423, 56128, 56418, 56128, 56421, 56128, 56430, 56128, 56423]);
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,50,93]), scotland.as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[44032,8204])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), UStr::new(&[97,65])).unwrap();
    });
}

#[test]
fn band_edges_and_gaps_exercise_every_range_arm() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.bandEdgesAndGapsExerciseEveryRangeArm", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.bandEdgesAndGapsExerciseEveryRangeArm", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[98,97,110,100,69,100,103,101,115,65,110,100,71,97,112,115,69,120,101,114,99,105,115,101,69,118,101,114,121,82,97,110,103,101,65,114,109]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), UStr::new(&[13,65])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), UStr::new(&[97,4352])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,44,32,51,93]), UStr::new(&[4352,4449,57344])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), TestHelpers::test_helpers_surrogate_text(&vec![55296, 57344]).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,51,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("a")); __s += TestHelpers::test_helpers_surrogate_text(&vec![56128, 56816]).as_ustr(); __s }).as_str()).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,51,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("a")); __s += TestHelpers::test_helpers_surrogate_text(&vec![56128, 56480]).as_ustr(); __s }).as_str()).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,44,32,51,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397]).as_ustr(); __s += &(UString::from("甲")); __s }).as_str()).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,44,32,52,93]), TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 55357, 56832]).as_ustr()).unwrap();
    });
}

#[test]
fn emoji_modifiers_only_attach_to_bases() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.emojiModifiersOnlyAttachToBases", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.emojiModifiersOnlyAttachToBases", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[101,109,111,106,105,77,111,100,105,102,105,101,114,115,79,110,108,121,65,116,116,97,99,104,84,111,66,97,115,101,115]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,52,93]), TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 55356, 57339]).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,53,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 55356, 57339]).as_ustr(); __s += &(UString::from("️")); __s }).as_str()).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,51,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("a")); __s += TestHelpers::test_helpers_surrogate_text(&vec![55356, 57339]).as_ustr(); __s }).as_str()).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397]).as_ustr()).unwrap();
    });
}

#[test]
fn zwj_chains_join_only_extended_pictographic() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.zwjChainsJoinOnlyExtendedPictographic", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.zwjChainsJoinOnlyExtendedPictographic", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[122,119,106,67,104,97,105,110,115,74,111,105,110,79,110,108,121,69,120,116,101,110,100,101,100,80,105,99,116,111,103,114,97,112,104,105,99]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,56,93]), TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 8205, 55357, 56425, 8205, 55357, 56422]).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,93]), UStr::new(&[97,8205])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,51,44,32,52,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425]).as_ustr(); __s += &(UString::from("‍a")); __s }).as_str()).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,44,32,51,93]), UStr::new(&[97,8205,97])).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,55,93]), TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 8205, 55357, 56397, 55356, 57339]).as_ustr()).unwrap();
    });
}

#[test]
fn unpaired_surrogates_fall_back_to_single_units() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.unpairedSurrogatesFallBackToSingleUnits", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.unpairedSurrogatesFallBackToSingleUnits", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[117,110,112,97,105,114,101,100,83,117,114,114,111,103,97,116,101,115,70,97,108,108,66,97,99,107,84,111,83,105,110,103,108,101,85,110,105,116,115]));
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,93]), TestHelpers::test_helpers_surrogate_text(&vec![97, 55296]).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,49,44,32,50,44,32,51,93]), TestHelpers::test_helpers_surrogate_text(&vec![97, 55296, 65]).as_ustr()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(UStr::new(&[91,48,44,32,50,44,32,51,93]), UString::from(format!("{}", { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]).as_ustr(); __s += &(UString::from("A")); __s }).as_str()).as_ustr()).unwrap();
    });
}

#[test]
fn code_point_at_compat_covers_every_surrogate_case() {
    testlib::record_not_applicable("org.tiqian.core.SourceInteractionBoundariesCoverageTest.codePointAtCompatCoversEverySurrogateCase", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.codePointAtCompatCoversEverySurrogateCase");
}

#[test]
fn range_boundaries_respect_the_requested_window() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.rangeBoundariesRespectTheRequestedWindow", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.rangeBoundariesRespectTheRequestedWindow", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[114,97,110,103,101,66,111,117,110,100,97,114,105,101,115,82,101,115,112,101,99,116,84,104,101,82,101,113,117,101,115,116,101,100,87,105,110,100,111,119]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,49,44,32,50,44,32,51,93]), SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(UStr::new(&[97,98,99,100]), TextRange::new(1u32, 3u32).unwrap())).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,50,93]), SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(UStr::new(&[97,98]), TextRange::new(5u32, 9u32).unwrap())).as_ustr(), None).unwrap();
        let emoji_b = { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]).as_ustr(); __s += &(UString::from("b")); __s };
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,48,44,32,50,44,32,51,93]), SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(emoji_b.as_ustr(), TextRange::new(0u32, 3u32).unwrap())).as_ustr(), None).unwrap();
    });
}

#[test]
fn coercion_honours_every_bias_and_edge_case() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.coercionHonoursEveryBiasAndEdgeCase", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.coercionHonoursEveryBiasAndEdgeCase", || {
        TestTraceRecorder::new(&(UStr::new(&[83,111,117,114,99,101,73,110,116,101,114,97,99,116,105,111,110,66,111,117,110,100,97,114,105,101,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[99,111,101,114,99,105,111,110,72,111,110,111,117,114,115,69,118,101,114,121,66,105,97,115,65,110,100,69,100,103,101,67,97,115,101]));
        let family = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56424, 8205, 55357, 56425, 8205, 55357, 56423, 8205, 55357, 56423]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(11, u_string::unit_count(&(family)), None).unwrap();
        let family_range = TextRange::new(0u32, 11u32).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(family.as_ustr(), 2, (family_range).clone(), SourceBoundaryBias::Nearest), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(family.as_ustr(), 2, (family_range).clone(), SourceBoundaryBias::Backward), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(11, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(family.as_ustr(), 2, (family_range).clone(), SourceBoundaryBias::Forward), None).unwrap();
        let emoji_b = { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]).as_ustr(); __s += &(UString::from("b")); __s };
        let emoji_range = TextRange::new(0u32, 3u32).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(emoji_b.as_ustr(), 2, (emoji_range).clone(), SourceBoundaryBias::Nearest), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(emoji_b.as_ustr(), 9, (emoji_range).clone(), SourceBoundaryBias::Backward), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(emoji_b.as_ustr(), 4294967295u32, (emoji_range).clone(), SourceBoundaryBias::Forward), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct SourceInteractionBoundariesCoverageTestHelpers;

impl SourceInteractionBoundariesCoverageTestHelpers {
    pub fn source_interaction_boundaries_coverage_test_helpers_boundaries(text: &UStr) -> Result<Vec<u32>, TextRangeError> {
        return Ok(SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(text, TextRange::new(0u32, u_string::unit_count(&(text)))?));
    }

    pub fn source_interaction_boundaries_coverage_test_helpers_assert_boundaries(expected: &UStr, text: &UStr) -> Result<(), SourceInteractionBoundariesCoverageTestAssertBoundariesFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(expected, SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_boundaries(text).map_err(|e| SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TextRangeErrorFault(e))?).as_ustr(), None).map_err(|e| SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn source_interaction_boundaries_coverage_test_helpers_render_ints(values: &Vec<u32>) -> UString {
        let mut output = UString::from("[").to_ustring();
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) > (0) {
                output += &(UString::from(", "));
            }
            output += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(values[usize::try_from(index).unwrap_or(0)])).as_str()));
            index = u32::wrapping_add(index, 1);
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += output.as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }
}
