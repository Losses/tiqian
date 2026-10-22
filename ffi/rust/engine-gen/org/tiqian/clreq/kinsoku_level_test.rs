#![cfg(test)]

use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::clreq::kinsoku_modes::KinsokuModes;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn none_forbids_nothing() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.noneForbidsNothing", "org.tiqian.clreq.KinsokuLevelTest.noneForbidsNothing", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[110,111,110,101,70,111,114,98,105,100,115,78,111,116,104,105,110,103]));
        let chars = vec![
    UString::from("。").to_ustring(),
    UString::from("，").to_ustring(),
    UString::from("、").to_ustring(),
    UString::from("”").to_ustring(),
    UString::from("）").to_ustring(),
    UString::from("·").to_ustring(),
    UString::from("／").to_ustring(),
    UString::from("—").to_ustring(),
    UString::from("…").to_ustring(),
    UString::from("“").to_ustring(),
    UString::from("（").to_ustring(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((chars.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let c = (chars[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(c.as_ustr(), KinsokuLevel::None), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += c.as_ustr(); __s += &(UString::from(" start@None")); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(c.as_ustr(), KinsokuLevel::None), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += c.as_ustr(); __s += &(UString::from(" end@None")); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn basic_forbids_pause_stops_closing_connectors_at_start_and_opening_at_end() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.basicForbidsPauseStopsClosingConnectorsAtStartAndOpeningAtEnd", "org.tiqian.clreq.KinsokuLevelTest.basicForbidsPauseStopsClosingConnectorsAtStartAndOpeningAtEnd", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[98,97,115,105,99,70,111,114,98,105,100,115,80,97,117,115,101,83,116,111,112,115,67,108,111,115,105,110,103,67,111,110,110,101,99,116,111,114,115,65,116,83,116,97,114,116,65,110,100,79,112,101,110,105,110,103,65,116,69,110,100]));
        let start_chars = vec![
    UString::from("。").to_ustring(),
    UString::from("，").to_ustring(),
    UString::from("、").to_ustring(),
    UString::from("：").to_ustring(),
    UString::from("；").to_ustring(),
    UString::from("！").to_ustring(),
    UString::from("？").to_ustring(),
    UString::from("”").to_ustring(),
    UString::from("）").to_ustring(),
    UString::from("】").to_ustring(),
    UString::from("·").to_ustring(),
    UString::from("～").to_ustring(),
    UString::from("／").to_ustring(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((start_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let c = (start_chars[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(c.as_ustr(), KinsokuLevel::Basic), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += c.as_ustr(); __s += &(UString::from(" start@Basic")); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let end_chars = vec![
    UString::from("“").to_ustring(),
    UString::from("（").to_ustring(),
    UString::from("《").to_ustring(),
    UString::from("「").to_ustring(),
    UString::from("【").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((end_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let c = (end_chars[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(c.as_ustr(), KinsokuLevel::Basic), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += c.as_ustr(); __s += &(UString::from(" end@Basic")); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8212]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8230]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(UStr::new(&[65295]), KinsokuLevel::Basic), None).unwrap();
    });
}

#[test]
fn gb_style_adds_separator_at_line_end() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.gbStyleAddsSeparatorAtLineEnd", "org.tiqian.clreq.KinsokuLevelTest.gbStyleAddsSeparatorAtLineEnd", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[103,98,83,116,121,108,101,65,100,100,115,83,101,112,97,114,97,116,111,114,65,116,76,105,110,101,69,110,100]));
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(UStr::new(&[65295]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(UStr::new(&[65295]), KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8212]), KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8230]), KinsokuLevel::GbStyle), None).unwrap();
    });
}

#[test]
fn strict_adds_dash_and_ellipsis_at_line_start() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.strictAddsDashAndEllipsisAtLineStart", "org.tiqian.clreq.KinsokuLevelTest.strictAddsDashAndEllipsisAtLineStart", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[115,116,114,105,99,116,65,100,100,115,68,97,115,104,65,110,100,69,108,108,105,112,115,105,115,65,116,76,105,110,101,83,116,97,114,116]));
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8212]), KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8212]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8230]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(UStr::new(&[8943]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(UStr::new(&[65295]), KinsokuLevel::Strict), None).unwrap();
    });
}

#[test]
fn profile_defaults_to_measure_adaptive() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.profileDefaultsToMeasureAdaptive", "org.tiqian.clreq.KinsokuLevelTest.profileDefaultsToMeasureAdaptive", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[112,114,111,102,105,108,101,68,101,102,97,117,108,116,115,84,111,77,101,97,115,117,114,101,65,100,97,112,116,105,118,101]));
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_is_measure_adaptive(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().kinsoku_mode).clone()), None).unwrap();
    });
}

#[test]
fn cjk_bracket_variants_classify_as_opening_and_closing() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.cjkBracketVariantsClassifyAsOpeningAndClosing", "org.tiqian.clreq.KinsokuLevelTest.cjkBracketVariantsClassifyAsOpeningAndClosing", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[99,106,107,66,114,97,99,107,101,116,86,97,114,105,97,110,116,115,67,108,97,115,115,105,102,121,65,115,79,112,101,110,105,110,103,65,110,100,67,108,111,115,105,110,103]));
        let opening = vec![
    UString::from("【").to_ustring(),
    UString::from("〔").to_ustring(),
    UString::from("〖").to_ustring(),
    UString::from("〘").to_ustring(),
    UString::from("〚").to_ustring(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((opening.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Opening, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((opening[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), Some(((opening[usize::try_from(index).unwrap_or(0)]).clone()).to_ustring())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let closing = vec![
    UString::from("】").to_ustring(),
    UString::from("〕").to_ustring(),
    UString::from("〗").to_ustring(),
    UString::from("〙").to_ustring(),
    UString::from("〛").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((closing.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Closing, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((closing[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), Some(((closing[usize::try_from(index).unwrap_or(0)]).clone()).to_ustring())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn exposes_unambiguous_ascii_point_marks_without_guessing_quotes_or_connectors() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.exposesUnambiguousAsciiPointMarksWithoutGuessingQuotesOrConnectors", "org.tiqian.clreq.KinsokuLevelTest.exposesUnambiguousAsciiPointMarksWithoutGuessingQuotesOrConnectors", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[101,120,112,111,115,101,115,85,110,97,109,98,105,103,117,111,117,115,65,115,99,105,105,80,111,105,110,116,77,97,114,107,115,87,105,116,104,111,117,116,71,117,101,115,115,105,110,103,81,117,111,116,101,115,79,114,67,111,110,110,101,99,116,111,114,115]));
        let included = vec![
    UString::from(",").to_ustring(),
    UString::from(".").to_ustring(),
    UString::from(":").to_ustring(),
    UString::from(";").to_ustring(),
    UString::from("!").to_ustring(),
    UString::from("?").to_ustring(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((included.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((included[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (included[usize::try_from(index).unwrap_or(0)]).clone().as_ustr(); __s += &(UString::from(" point mark")); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let excluded = vec![
    UString::from("\"").to_ustring(),
    UString::from("'").to_ustring(),
    UString::from("-").to_ustring(),
    UString::from("/").to_ustring(),
    UString::from("~").to_ustring(),
    UString::from("%").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((excluded.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((excluded[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (excluded[usize::try_from(index).unwrap_or(0)]).clone().as_ustr(); __s += &(UString::from(" excluded")); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn measure_adaptive_resolves_per_line_width() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.measureAdaptiveResolvesPerLineWidth", "org.tiqian.clreq.KinsokuLevelTest.measureAdaptiveResolvesPerLineWidth", || {
        TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,76,101,118,101,108,84,101,115,116]))).section(UStr::new(&[109,101,97,115,117,114,101,65,100,97,112,116,105,118,101,82,101,115,111,108,118,101,115,80,101,114,76,105,110,101,87,105,100,116,104]));
        let m = KinsokuMode::MeasureAdaptive { hang_below_em: 14.0f64, gb_above_em: 24.0f64, strict_above_em: 32.0f64 };
        let narrow = KinsokuModes::kinsoku_modes_resolve((m).clone(), 10.0f64);
        let _ = TracedAssertions::traced_assertions_assert_equals_kinsoku_level(KinsokuLevel::Basic, narrow.level, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_hanging_punctuation_style(HangingPunctuationStyle::PauseStops, narrow.hanging, None).unwrap();
        let medium = KinsokuModes::kinsoku_modes_resolve((m).clone(), 20.0f64);
        let _ = TracedAssertions::traced_assertions_assert_equals_kinsoku_level(KinsokuLevel::Basic, medium.level, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_hanging_punctuation_style(HangingPunctuationStyle::Disabled, medium.hanging, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_kinsoku_level(KinsokuLevel::GbStyle, KinsokuModes::kinsoku_modes_resolve((m).clone(), 28.0f64).level, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_kinsoku_level(KinsokuLevel::Strict, KinsokuModes::kinsoku_modes_resolve((m).clone(), 40.0f64).level, None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct KinsokuLevelTestHelpers;

impl KinsokuLevelTestHelpers {
    pub fn kinsoku_level_test_helpers_start(char: &UStr, level: KinsokuLevel) -> bool {
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(char, level);
    }

    pub fn kinsoku_level_test_helpers_end(char: &UStr, level: KinsokuLevel) -> bool {
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(char, level);
    }

    pub fn kinsoku_level_test_helpers_is_measure_adaptive(mode: KinsokuMode) -> bool {
        return match mode {
            KinsokuMode::Fixed { .. } => false,
            KinsokuMode::MeasureAdaptive { .. } => true,
        };
    }
}
