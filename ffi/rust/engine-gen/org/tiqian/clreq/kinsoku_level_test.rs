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


#[test]
fn none_forbids_nothing() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.noneForbidsNothing", "org.tiqian.clreq.KinsokuLevelTest.noneForbidsNothing", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"noneForbidsNothing");
        let chars = vec![
    "。".to_string(),
    "，".to_string(),
    "、".to_string(),
    "”".to_string(),
    "）".to_string(),
    "·".to_string(),
    "／".to_string(),
    "—".to_string(),
    "…".to_string(),
    "“".to_string(),
    "（".to_string(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((chars.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let c = (chars[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(c.as_str(), KinsokuLevel::None), Some((format!("{}{}",
            c,
            " start@None"
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(c.as_str(), KinsokuLevel::None), Some((format!("{}{}",
            c,
            " end@None"
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn basic_forbids_pause_stops_closing_connectors_at_start_and_opening_at_end() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.basicForbidsPauseStopsClosingConnectorsAtStartAndOpeningAtEnd", "org.tiqian.clreq.KinsokuLevelTest.basicForbidsPauseStopsClosingConnectorsAtStartAndOpeningAtEnd", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"basicForbidsPauseStopsClosingConnectorsAtStartAndOpeningAtEnd");
        let start_chars = vec![
    "。".to_string(),
    "，".to_string(),
    "、".to_string(),
    "：".to_string(),
    "；".to_string(),
    "！".to_string(),
    "？".to_string(),
    "”".to_string(),
    "）".to_string(),
    "】".to_string(),
    "·".to_string(),
    "～".to_string(),
    "／".to_string(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((start_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let c = (start_chars[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(c.as_str(), KinsokuLevel::Basic), Some((format!("{}{}",
            c,
            " start@Basic"
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let end_chars = vec!["“".to_string(), "（".to_string(), "《".to_string(), "「".to_string(), "【".to_string()];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((end_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let c = (end_chars[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(c.as_str(), KinsokuLevel::Basic), Some((format!("{}{}",
            c,
            " end@Basic"
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"—", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"…", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(&"／", KinsokuLevel::Basic), None).unwrap();
    });
}

#[test]
fn gb_style_adds_separator_at_line_end() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.gbStyleAddsSeparatorAtLineEnd", "org.tiqian.clreq.KinsokuLevelTest.gbStyleAddsSeparatorAtLineEnd", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"gbStyleAddsSeparatorAtLineEnd");
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(&"／", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(&"／", KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"—", KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"…", KinsokuLevel::GbStyle), None).unwrap();
    });
}

#[test]
fn strict_adds_dash_and_ellipsis_at_line_start() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.strictAddsDashAndEllipsisAtLineStart", "org.tiqian.clreq.KinsokuLevelTest.strictAddsDashAndEllipsisAtLineStart", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"strictAddsDashAndEllipsisAtLineStart");
        let _ = TracedAssertions::traced_assertions_assert_false(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"—", KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"—", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"…", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_start(&"⋯", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_end(&"／", KinsokuLevel::Strict), None).unwrap();
    });
}

#[test]
fn profile_defaults_to_measure_adaptive() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.profileDefaultsToMeasureAdaptive", "org.tiqian.clreq.KinsokuLevelTest.profileDefaultsToMeasureAdaptive", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"profileDefaultsToMeasureAdaptive");
        let _ = TracedAssertions::traced_assertions_assert_true(KinsokuLevelTestHelpers::kinsoku_level_test_helpers_is_measure_adaptive(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().kinsoku_mode).clone()), None).unwrap();
    });
}

#[test]
fn cjk_bracket_variants_classify_as_opening_and_closing() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.cjkBracketVariantsClassifyAsOpeningAndClosing", "org.tiqian.clreq.KinsokuLevelTest.cjkBracketVariantsClassifyAsOpeningAndClosing", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"cjkBracketVariantsClassifyAsOpeningAndClosing");
        let opening = vec!["【".to_string(), "〔".to_string(), "〖".to_string(), "〘".to_string(), "〚".to_string()];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((opening.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Opening, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((opening[usize::try_from(index).unwrap_or(0)]).clone().as_str()),
Some(((opening[usize::try_from(index).unwrap_or(0)]).clone()).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let closing = vec!["】".to_string(), "〕".to_string(), "〗".to_string(), "〙".to_string(), "〛".to_string()];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((closing.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Closing, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((closing[usize::try_from(index).unwrap_or(0)]).clone().as_str()),
Some(((closing[usize::try_from(index).unwrap_or(0)]).clone()).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn exposes_unambiguous_ascii_point_marks_without_guessing_quotes_or_connectors() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.exposesUnambiguousAsciiPointMarksWithoutGuessingQuotesOrConnectors", "org.tiqian.clreq.KinsokuLevelTest.exposesUnambiguousAsciiPointMarksWithoutGuessingQuotesOrConnectors", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"exposesUnambiguousAsciiPointMarksWithoutGuessingQuotesOrConnectors");
        let included = vec![
    ",".to_string(),
    ".".to_string(),
    ":".to_string(),
    ";".to_string(),
    "!".to_string(),
    "?".to_string(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((included.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((included[usize::try_from(index).unwrap_or(0)]).clone().as_str()), Some((format!("{}{}",
            (included[usize::try_from(index).unwrap_or(0)]).clone(),
            " point mark"
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let excluded = vec![
    "\"".to_string(),
    "'".to_string(),
    "-".to_string(),
    "/".to_string(),
    "~".to_string(),
    "%".to_string(),
];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((excluded.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((excluded[usize::try_from(index).unwrap_or(0)]).clone().as_str()), Some((format!("{}{}",
            (excluded[usize::try_from(index).unwrap_or(0)]).clone(),
            " excluded"
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn measure_adaptive_resolves_per_line_width() {
    testlib::run("org.tiqian.clreq.KinsokuLevelTest.measureAdaptiveResolvesPerLineWidth", "org.tiqian.clreq.KinsokuLevelTest.measureAdaptiveResolvesPerLineWidth", || {
        TestTraceRecorder::new("KinsokuLevelTest").section(&"measureAdaptiveResolvesPerLineWidth");
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
    pub fn kinsoku_level_test_helpers_start(char: &str, level: KinsokuLevel) -> bool {
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(char, level);
    }

    pub fn kinsoku_level_test_helpers_end(char: &str, level: KinsokuLevel) -> bool {
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(char, level);
    }

    pub fn kinsoku_level_test_helpers_is_measure_adaptive(mode: KinsokuMode) -> bool {
        return match mode {
            KinsokuMode::Fixed { .. } => false,
            KinsokuMode::MeasureAdaptive { .. } => true,
        };
    }
}
