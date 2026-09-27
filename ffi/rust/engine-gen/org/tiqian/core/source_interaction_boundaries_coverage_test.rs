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


#[derive(Debug, Clone, PartialEq)]
pub enum SourceInteractionBoundariesCoverageTestRangeBoundariesRespectTheRequestedWindowFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"crlfStaysOneUnit");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &concat!("\r\n",
"")).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1]", &"\r").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", &concat!("a\n",
"")).unwrap();
    });
}

#[test]
fn regional_indicators_pair_up() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.regionalIndicatorsPairUp", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.regionalIndicatorsPairUp", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"regionalIndicatorsPairUp");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 4]", TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806, 55356, 56808]).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 4, 6]", TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806, 55356, 56806, 55356, 56806]).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2, 3]", format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806]),
            "A"
        ).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", TestHelpers::test_helpers_surrogate_text(&vec![55356, 56806]).as_str()).unwrap();
    });
}

#[test]
fn hangul_jamo_runs_merge_into_syllable_blocks() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.hangulJamoRunsMergeIntoSyllableBlocks", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.hangulJamoRunsMergeIntoSyllableBlocks", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"hangulJamoRunsMergeIntoSyllableBlocks");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 3]", &"ᄀ가").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 3]", &"각").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 4]", &"각ᆨ").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", &"ᄀA").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2, 3]", &"가A").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"ꥠᅡ").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"ᄀힰ").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 3]", &"가ퟋ").unwrap();
    });
}

#[test]
fn precomposed_hangul_syllables_absorb_jamo() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.precomposedHangulSyllablesAbsorbJamo", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.precomposedHangulSyllablesAbsorbJamo", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"precomposedHangulSyllablesAbsorbJamo");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 3]", &"가ᅡᆨ").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"각ᆨ").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", &"각A").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"각").unwrap();
    });
}

#[test]
fn extenders_attach_to_the_preceding_unit() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.extendersAttachToThePrecedingUnit", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.extendersAttachToThePrecedingUnit", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"extendersAttachToThePrecedingUnit");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"á").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"a️").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 3]", format!("{}{}",
            "a",
            TestHelpers::test_helpers_surrogate_text(&vec![56128, 56576])
        ).as_str()).unwrap();
        let scotland = TestHelpers::test_helpers_surrogate_text(&vec![55356, 57332, 56128, 56423, 56128, 56418, 56128, 56421, 56128, 56430, 56128, 56423]);
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 12]", scotland.as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"가‌").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", &"aA").unwrap();
    });
}

#[test]
fn band_edges_and_gaps_exercise_every_range_arm() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.bandEdgesAndGapsExerciseEveryRangeArm", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.bandEdgesAndGapsExerciseEveryRangeArm", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"bandEdgesAndGapsExerciseEveryRangeArm");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", &"\rA").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", &"aᄀ").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2, 3]", &"가").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", TestHelpers::test_helpers_surrogate_text(&vec![55296, 57344]).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 3]", format!("{}{}",
            "a",
            TestHelpers::test_helpers_surrogate_text(&vec![56128, 56816])
        ).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 3]", format!("{}{}",
            "a",
            TestHelpers::test_helpers_surrogate_text(&vec![56128, 56480])
        ).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2, 3]", format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397]),
            "甲"
        ).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2, 4]", TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 55357, 56832]).as_str()).unwrap();
    });
}

#[test]
fn emoji_modifiers_only_attach_to_bases() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.emojiModifiersOnlyAttachToBases", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.emojiModifiersOnlyAttachToBases", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"emojiModifiersOnlyAttachToBases");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 4]", TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 55356, 57339]).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 5]", format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 55356, 57339]),
            "️"
        ).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 3]", format!("{}{}",
            "a",
            TestHelpers::test_helpers_surrogate_text(&vec![55356, 57339])
        ).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397]).as_str()).unwrap();
    });
}

#[test]
fn zwj_chains_join_only_extended_pictographic() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.zwjChainsJoinOnlyExtendedPictographic", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.zwjChainsJoinOnlyExtendedPictographic", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"zwjChainsJoinOnlyExtendedPictographic");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 8]", TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 8205, 55357, 56425, 8205, 55357, 56422]).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2]", &"a‍").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 3, 4]", format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425]),
            "‍a"
        ).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2, 3]", &"a‍a").unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 7]", TestHelpers::test_helpers_surrogate_text(&vec![55357, 56397, 8205, 55357, 56397, 55356, 57339]).as_str()).unwrap();
    });
}

#[test]
fn unpaired_surrogates_fall_back_to_single_units() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.unpairedSurrogatesFallBackToSingleUnits", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.unpairedSurrogatesFallBackToSingleUnits", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"unpairedSurrogatesFallBackToSingleUnits");
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2]", TestHelpers::test_helpers_surrogate_text(&vec![97, 55296]).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 1, 2, 3]", TestHelpers::test_helpers_surrogate_text(&vec![97, 55296, 65]).as_str()).unwrap();
        let _ = SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_assert_boundaries(&"[0, 2, 3]", format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]),
            "A"
        ).as_str()).unwrap();
    });
}

#[test]
fn code_point_at_compat_covers_every_surrogate_case() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.codePointAtCompatCoversEverySurrogateCase", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.codePointAtCompatCoversEverySurrogateCase", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"codePointAtCompatCoversEverySurrogateCase");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(97, SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(&"a", 0, 1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(128512, SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]).as_str(), 0, 2), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(55296, SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(TestHelpers::test_helpers_surrogate_text(&vec![97, 55296]).as_str(), 1, 2), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(55296, SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(TestHelpers::test_helpers_surrogate_text(&vec![97, 55296, 65]).as_str(), 1, 3), None).unwrap();
    });
}

#[test]
fn range_boundaries_respect_the_requested_window() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.rangeBoundariesRespectTheRequestedWindow", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.rangeBoundariesRespectTheRequestedWindow", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"rangeBoundariesRespectTheRequestedWindow");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[1, 2, 3]",
SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(&"abcd", TextRange::new(1u32, 3u32).unwrap())).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[2]",
SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(&"ab", TextRange::new(5u32, 9u32).unwrap())).as_str(), None).unwrap();
        let emoji_b = format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]),
            "b"
        );
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[0, 2, 3]",
SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(emoji_b.as_str(), TextRange::new(0u32, 3u32).unwrap())).as_str(), None).unwrap();
    });
}

#[test]
fn coercion_honours_every_bias_and_edge_case() {
    testlib::run("org.tiqian.core.SourceInteractionBoundariesCoverageTest.coercionHonoursEveryBiasAndEdgeCase", "org.tiqian.core.SourceInteractionBoundariesCoverageTest.coercionHonoursEveryBiasAndEdgeCase", || {
        TestTraceRecorder::new("SourceInteractionBoundariesCoverageTest").section(&"coercionHonoursEveryBiasAndEdgeCase");
        let family = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56424, 8205, 55357, 56425, 8205, 55357, 56423, 8205, 55357, 56423]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(11, u_string::unit_count(&(family)), None).unwrap();
        let family_range = TextRange::new(0u32, 11u32).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(family.as_str(), 2, (family_range).clone(), SourceBoundaryBias::Nearest), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(family.as_str(), 2, (family_range).clone(), SourceBoundaryBias::Backward), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(11, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(family.as_str(), 2, (family_range).clone(), SourceBoundaryBias::Forward), None).unwrap();
        let emoji_b = format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]),
            "b"
        );
        let emoji_range = TextRange::new(0u32, 3u32).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(emoji_b.as_str(), 2, (emoji_range).clone(), SourceBoundaryBias::Nearest), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(emoji_b.as_str(), 9, (emoji_range).clone(), SourceBoundaryBias::Backward), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(emoji_b.as_str(), 4294967295u32, (emoji_range).clone(), SourceBoundaryBias::Forward), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct SourceInteractionBoundariesCoverageTestHelpers;

impl SourceInteractionBoundariesCoverageTestHelpers {
    pub fn source_interaction_boundaries_coverage_test_helpers_boundaries(text: &str) -> Result<Vec<u32>, TextRangeError> {
        return Ok(SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(text, TextRange::new(0u32, u_string::unit_count(&(text)))?));
    }

    pub fn source_interaction_boundaries_coverage_test_helpers_assert_boundaries(expected: &str, text: &str) -> Result<(), SourceInteractionBoundariesCoverageTestAssertBoundariesFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(expected,
SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_render_ints(&SourceInteractionBoundariesCoverageTestHelpers::source_interaction_boundaries_coverage_test_helpers_boundaries(text).map_err(|e|
SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TextRangeErrorFault(e))?).as_str(), None).map_err(|e| SourceInteractionBoundariesCoverageTestAssertBoundariesFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn source_interaction_boundaries_coverage_test_helpers_render_ints(values: &Vec<u32>) -> String {
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
