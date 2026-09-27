#![cfg(test)]

use crate::org::tiqian::clreq::bopomofo_parser::BopomofoParser;
use crate::org::tiqian::clreq::bopomofo_reading::BopomofoReading;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_punctuation_advance_policy::ClreqPunctuationAdvancePolicy;
use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::clreq_region::ClreqRegion;
use crate::org::tiqian::clreq::clreq_strictness::ClreqStrictness;
use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_glue_placements::PunctuationGluePlacements;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[test]
fn test_bopomofo_models_and_parser() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testBopomofoModelsAndParser", "org.tiqian.clreq.ClreqProfileCoverageTest.testBopomofoModelsAndParser", || {
        TestTraceRecorder::new("ClreqProfileCoverageTest").section(&"testBopomofoModelsAndParser");
        let tones = vec![
    BopomofoTone::Yinping,
    BopomofoTone::Yangping,
    BopomofoTone::Shang,
    BopomofoTone::Qu,
    BopomofoTone::Neutral,
    BopomofoTone::Ru,
];
        let mut tone_index = 0u32;
        while (i32::from_ne_bytes((tone_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((tones.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let tone = tones[usize::try_from(tone_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { tone.name().to_string() }.as_str(), None).unwrap();
            tone_index = u32::wrapping_add(tone_index, 1);
        }
        let reading = BopomofoReading::new(vec!["ㄅ".to_string(), "ㄚ".to_string()].to_vec(), BopomofoTone::Yangping);
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄅ".to_string(), "ㄚ".to_string()], &reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yangping, reading.tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading((reading).clone(), reading.copy(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(reading.hash_code() == reading.copy().hash_code(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&reading.to_string(), "BopomofoReading", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let empty_reading = BopomofoParser::bopomofo_parser_parse(&"");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![], &empty_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, empty_reading.tone, None).unwrap();
        let neutral_reading = BopomofoParser::bopomofo_parser_parse(&"˙ㄇㄚ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄇ".to_string(), "ㄚ".to_string()], &neutral_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Neutral, neutral_reading.tone, None).unwrap();
        let yangping_reading = BopomofoParser::bopomofo_parser_parse(&"ㄇㄚˊ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄇ".to_string(), "ㄚ".to_string()], &yangping_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yangping, yangping_reading.tone, None).unwrap();
        let shang_reading = BopomofoParser::bopomofo_parser_parse(&"ㄇㄚˇ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄇ".to_string(), "ㄚ".to_string()], &shang_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Shang, shang_reading.tone, None).unwrap();
        let qu_reading = BopomofoParser::bopomofo_parser_parse(&"ㄇㄚˋ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄇ".to_string(), "ㄚ".to_string()], &qu_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Qu, qu_reading.tone, None).unwrap();
        let explicit_yinping = BopomofoParser::bopomofo_parser_parse(&"ㄇㄚˉ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄇ".to_string(), "ㄚ".to_string()], &explicit_yinping.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, explicit_yinping.tone, None).unwrap();
        let default_yinping = BopomofoParser::bopomofo_parser_parse(&"ㄇㄚ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄇ".to_string(), "ㄚ".to_string()], &default_yinping.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, default_yinping.tone, None).unwrap();
    });
}

#[test]
fn test_clreq_profile_and_resolver() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testClreqProfileAndResolver", "org.tiqian.clreq.ClreqProfileCoverageTest.testClreqProfileAndResolver", || {
        TestTraceRecorder::new("ClreqProfileCoverageTest").section(&"testClreqProfileAndResolver");
        let strictnesses = vec![ClreqStrictness::Loose, ClreqStrictness::Normal, ClreqStrictness::Strict];
        let mut strictness_index = 0u32;
        while (i32::from_ne_bytes((strictness_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((strictnesses.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let strictness = strictnesses[usize::try_from(strictness_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { strictness.name().to_string() }.as_str(), None).unwrap();
            strictness_index = u32::wrapping_add(strictness_index, 1);
        }
        let regions = vec![ClreqRegion::Mainland, ClreqRegion::Taiwan, ClreqRegion::HongKong, ClreqRegion::Custom];
        let mut region_index = 0u32;
        while (i32::from_ne_bytes((region_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((regions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let region = regions[usize::try_from(region_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { region.name().to_string() }.as_str(), None).unwrap();
            region_index = u32::wrapping_add(region_index, 1);
        }
        let glyph_policies = vec![
    CjkPunctuationGlyphPolicy::PreserveInput,
    CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints,
    CjkPunctuationGlyphPolicy::ForceClreqRecommendedCodepoints,
];
        let mut policy_index = 0u32;
        while (i32::from_ne_bytes((policy_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((glyph_policies.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let policy = glyph_policies[usize::try_from(policy_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { policy.name().to_string() }.as_str(), None).unwrap();
            policy_index = u32::wrapping_add(policy_index, 1);
        }
        let classes = vec![
    PunctuationClass::Opening,
    PunctuationClass::Closing,
    PunctuationClass::PauseOrStop,
    PunctuationClass::MiddleDot,
    PunctuationClass::Interpunct,
    PunctuationClass::Connector,
    PunctuationClass::Solidus,
    PunctuationClass::Ellipsis,
    PunctuationClass::Dash,
    PunctuationClass::Other,
];
        let mut class_index = 0u32;
        while (i32::from_ne_bytes((class_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((classes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let cls = classes[usize::try_from(class_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { cls.name().to_string() }.as_str(), None).unwrap();
            class_index = u32::wrapping_add(class_index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_contains_int(&crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION, 8212), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"clreq-mainland-horizontal", ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().id).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"clreq-taiwan-horizontal", ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_TAIWAN_HORIZONTAL).clone().id).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"clreq-hongkong-horizontal", ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_HONG_KONG_HORIZONTAL).clone().id).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Mainland), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Taiwan), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::HongKong), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Custom), None).unwrap();
        let resolver = BuiltInClreqProfileResolver::new();
        let resolved_built_in = resolver.resolve((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_clreq_profile((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone(), (resolved_built_in).clone(), None).unwrap();
        let resolved_mainland_id = resolver.resolve(LayoutProfileId::new("clreq-mainland-horizontal"));
        let _ = TracedAssertions::traced_assertions_assert_equals_clreq_profile((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone(), (resolved_mainland_id).clone(), None).unwrap();
        let resolved_other_id = resolver.resolve(LayoutProfileId::new("other-profile"));
        let _ = TracedAssertions::traced_assertions_assert_equals_clreq_profile((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone(), (resolved_other_id).clone(), None).unwrap();
    });
}

#[test]
fn test_clreq_punctuation_policies_and_classification() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testClreqPunctuationPoliciesAndClassification", "org.tiqian.clreq.ClreqProfileCoverageTest.testClreqPunctuationPoliciesAndClassification", || {
        TestTraceRecorder::new("ClreqProfileCoverageTest").section(&"testClreqPunctuationPoliciesAndClassification");
        let point_marks = vec![
    ",".to_string(),
    ".".to_string(),
    ":".to_string(),
    ";".to_string(),
    "!".to_string(),
    "?".to_string(),
];
        let mut point_index = 0u32;
        while (i32::from_ne_bytes((point_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((point_marks.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((point_marks[usize::try_from(point_index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            point_index = u32::wrapping_add(point_index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark(&"a"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark(&"，"), None).unwrap();
        let opening_chars = vec![
    "“".to_string(),
    "‘".to_string(),
    "（".to_string(),
    "《".to_string(),
    "〈".to_string(),
    "「".to_string(),
    "『".to_string(),
    "【".to_string(),
    "〔".to_string(),
    "〖".to_string(),
    "〘".to_string(),
    "〚".to_string(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((opening_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Opening, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((opening_chars[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let closing_chars = vec![
    "”".to_string(),
    "’".to_string(),
    "）".to_string(),
    "》".to_string(),
    "〉".to_string(),
    "」".to_string(),
    "』".to_string(),
    "】".to_string(),
    "〕".to_string(),
    "〗".to_string(),
    "〙".to_string(),
    "〛".to_string(),
];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((closing_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Closing, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((closing_chars[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let pause_or_stop_chars = vec![
    "，".to_string(),
    "、".to_string(),
    "。".to_string(),
    "；".to_string(),
    "：".to_string(),
    "！".to_string(),
    "？".to_string(),
];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pause_or_stop_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::PauseOrStop, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((pause_or_stop_chars[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::MiddleDot, ClreqPunctuationPolicies::clreq_punctuation_policies_classify(&"·"), None).unwrap();
        let interpuncts = vec!["・".to_string(), "‧".to_string(), "•".to_string()];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((interpuncts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Interpunct, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((interpuncts[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let connectors = vec!["～".to_string(), "~".to_string(), "-".to_string(), "–".to_string()];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((connectors.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Connector, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((connectors[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let solidi = vec!["/".to_string(), "／".to_string()];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((solidi.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Solidus, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((solidi[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let ellipses = vec!["…".to_string(), "⋯".to_string()];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ellipses.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Ellipsis, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((ellipses[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let dashes = vec!["—".to_string(), "⸺".to_string()];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((dashes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Dash, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((dashes[usize::try_from(index).unwrap_or(0)]).clone().as_str()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Other, ClreqPunctuationPolicies::clreq_punctuation_policies_classify(&"中"), None).unwrap();
    });
}

#[test]
fn test_forced_half_width_and_policy_for() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testForcedHalfWidthAndPolicyFor", "org.tiqian.clreq.ClreqProfileCoverageTest.testForcedHalfWidthAndPolicyFor", || {
        TestTraceRecorder::new("ClreqProfileCoverageTest").section(&"testForcedHalfWidthAndPolicyFor");
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"-", PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"–", PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))), None).unwrap();
        let gb_policy = PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(true));
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"～", (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"·", (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"•", (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"/", (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"，", (gb_policy).clone()), None).unwrap();
        let kaiming_policy = PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::Kaiming), Some(false));
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"（", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"）", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"，", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"；", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"。", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"！", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"？", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"．", (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(&"中", (kaiming_policy).clone()), None).unwrap();
        let dash2_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"⸺");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, dash2_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, dash2_policy.default_advance_em, None).unwrap();
        let hyphen_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"-");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, hyphen_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, hyphen_policy.default_advance_em, None).unwrap();
        let comma_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"，");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, comma_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, comma_policy.default_advance_em, None).unwrap();
        let open_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"（");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, open_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, open_policy.default_advance_em, None).unwrap();
        let close_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"）");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, close_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, close_policy.default_advance_em, None).unwrap();
        let han_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(&"字");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, han_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, han_policy.default_advance_em, None).unwrap();
    });
}

#[test]
fn test_forbidden_at_line_start_and_end() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testForbiddenAtLineStartAndEnd", "org.tiqian.clreq.ClreqProfileCoverageTest.testForbiddenAtLineStartAndEnd", || {
        TestTraceRecorder::new("ClreqProfileCoverageTest").section(&"testForbiddenAtLineStartAndEnd");
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"，", KinsokuLevel::None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"（", KinsokuLevel::None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"，", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"）", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"～", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"·", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"•", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"/", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"—", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"—", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"…", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"…", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"（", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"字", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"（", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"/", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"/", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"）", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"字", KinsokuLevel::Strict), None).unwrap();
    });
}

#[test]
fn test_punctuation_advance_and_substitutor() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testPunctuationAdvanceAndSubstitutor", "org.tiqian.clreq.ClreqProfileCoverageTest.testPunctuationAdvanceAndSubstitutor", || {
        TestTraceRecorder::new("ClreqProfileCoverageTest").section(&"testPunctuationAdvanceAndSubstitutor");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(&"⸺", &"⸺"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(&"—", &"⸺"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(&"⸺", &"——"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(&"abc", &"abc"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(&"😀", &"dummy"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_surrogate_text(&vec![55296]).as_str(), &"dummy"),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_surrogate_text(&vec![55296, 65]).as_str(), &"dummy"),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_surrogate_text(&vec![55296, 57344]).as_str(), &"dummy"),
None).unwrap();
        let preserve_substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreserveInput));
        let preserve_res = preserve_substitutor.substitute(&"……").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"……", (preserve_res.display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&(preserve_res.reason).to_string(), "preserve", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let prefer_substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints));
        let prefer_res = prefer_substitutor.substitute(&"——").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"——", (prefer_res.source_text).to_string().as_str(), None).unwrap();
        let force_substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::ForceClreqRecommendedCodepoints));
        let force_res = force_substitutor.substitute(&"abc").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"abc", (force_res.display_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&(force_res.reason).to_string(), "preserve", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct ClreqProfileCoverageTestHelpers;

impl ClreqProfileCoverageTestHelpers {
    pub fn clreq_profile_coverage_test_helpers_surrogate_text(codes: &Vec<u32>) -> String {
        let mut output = String::new();
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            output += &(if codes[usize::try_from(index).unwrap_or(0)] > 0xFFFF { String::from_utf16(&[0xD800 + (((codes[usize::try_from(index).unwrap_or(0)]) - 0x10000) >> 10) as u16, 0xDC00 + (((codes[usize::try_from(index).unwrap_or(0)]) - 0x10000) & 0x3FF) as u16]).unwrap() }
else { String::from_utf16_lossy(&[(codes[usize::try_from(index).unwrap_or(0)]) as u16]) });
            index = u32::wrapping_add(index, 1);
        }
        return output;
    }

    pub fn clreq_profile_coverage_test_helpers_contains_int(values: &[u32], value: u32) -> bool {
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if values[usize::try_from(index).unwrap_or(0)] == value {
                return true;
            }
            index = u32::wrapping_add(index, 1);
        }
        return false;
    }
}
