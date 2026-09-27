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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn test_bopomofo_models_and_parser() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testBopomofoModelsAndParser", "org.tiqian.clreq.ClreqProfileCoverageTest.testBopomofoModelsAndParser", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,114,111,102,105,108,101,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,66,111,112,111,109,111,102,111,77,111,100,101,108,115,65,110,100,80,97,114,115,101,114]));
        let tones = vec![
    BopomofoTone::Yinping,
    BopomofoTone::Yangping,
    BopomofoTone::Shang,
    BopomofoTone::Qu,
    BopomofoTone::Neutral,
    BopomofoTone::Ru,
];
        let mut tone_index = 0u32;
        while (i32::from_ne_bytes(((tone_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((tones.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let tone = tones[usize::try_from(tone_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(tone.name()) }.as_ustr(), None).unwrap();
            tone_index = u32::wrapping_add(tone_index, 1);
        }
        let reading = BopomofoReading::new(vec![UString::from("ㄅ").to_ustring(), UString::from("ㄚ").to_ustring()].to_vec(), BopomofoTone::Yangping);
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄅ").to_ustring(), UString::from("ㄚ").to_ustring()], &reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yangping, reading.tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading((reading).clone(), reading.copy(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(reading.hash_code() == reading.copy().hash_code(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", reading.to_string()).as_str()), UString::from("BopomofoReading").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let empty_reading = BopomofoParser::bopomofo_parser_parse(UStr::new(&[]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![], &empty_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, empty_reading.tone, None).unwrap();
        let neutral_reading = BopomofoParser::bopomofo_parser_parse(UStr::new(&[729,12551,12570]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄇ").to_ustring(), UString::from("ㄚ").to_ustring()], &neutral_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Neutral, neutral_reading.tone, None).unwrap();
        let yangping_reading = BopomofoParser::bopomofo_parser_parse(UStr::new(&[12551,12570,714]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄇ").to_ustring(), UString::from("ㄚ").to_ustring()], &yangping_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yangping, yangping_reading.tone, None).unwrap();
        let shang_reading = BopomofoParser::bopomofo_parser_parse(UStr::new(&[12551,12570,711]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄇ").to_ustring(), UString::from("ㄚ").to_ustring()], &shang_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Shang, shang_reading.tone, None).unwrap();
        let qu_reading = BopomofoParser::bopomofo_parser_parse(UStr::new(&[12551,12570,715]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄇ").to_ustring(), UString::from("ㄚ").to_ustring()], &qu_reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Qu, qu_reading.tone, None).unwrap();
        let explicit_yinping = BopomofoParser::bopomofo_parser_parse(UStr::new(&[12551,12570,713]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄇ").to_ustring(), UString::from("ㄚ").to_ustring()], &explicit_yinping.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, explicit_yinping.tone, None).unwrap();
        let default_yinping = BopomofoParser::bopomofo_parser_parse(UStr::new(&[12551,12570]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄇ").to_ustring(), UString::from("ㄚ").to_ustring()], &default_yinping.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, default_yinping.tone, None).unwrap();
    });
}

#[test]
fn test_clreq_profile_and_resolver() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testClreqProfileAndResolver", "org.tiqian.clreq.ClreqProfileCoverageTest.testClreqProfileAndResolver", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,114,111,102,105,108,101,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,67,108,114,101,113,80,114,111,102,105,108,101,65,110,100,82,101,115,111,108,118,101,114]));
        let strictnesses = vec![ClreqStrictness::Loose, ClreqStrictness::Normal, ClreqStrictness::Strict];
        let mut strictness_index = 0u32;
        while (i32::from_ne_bytes(((strictness_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((strictnesses.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let strictness = strictnesses[usize::try_from(strictness_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(strictness.name()) }.as_ustr(), None).unwrap();
            strictness_index = u32::wrapping_add(strictness_index, 1);
        }
        let regions = vec![ClreqRegion::Mainland, ClreqRegion::Taiwan, ClreqRegion::HongKong, ClreqRegion::Custom];
        let mut region_index = 0u32;
        while (i32::from_ne_bytes(((region_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((regions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let region = regions[usize::try_from(region_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(region.name()) }.as_ustr(), None).unwrap();
            region_index = u32::wrapping_add(region_index, 1);
        }
        let glyph_policies = vec![
    CjkPunctuationGlyphPolicy::PreserveInput,
    CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints,
    CjkPunctuationGlyphPolicy::ForceClreqRecommendedCodepoints,
];
        let mut policy_index = 0u32;
        while (i32::from_ne_bytes(((policy_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((glyph_policies.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let policy = glyph_policies[usize::try_from(policy_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(policy.name()) }.as_ustr(), None).unwrap();
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
        while (i32::from_ne_bytes(((class_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((classes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let cls = classes[usize::try_from(class_index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(cls.name()) }.as_ustr(), None).unwrap();
            class_index = u32::wrapping_add(class_index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_contains_int(&crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION, 8212), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,108,114,101,113,45,109,97,105,110,108,97,110,100,45,104,111,114,105,122,111,110,116,97,108]), ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().id).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,108,114,101,113,45,116,97,105,119,97,110,45,104,111,114,105,122,111,110,116,97,108]), ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_TAIWAN_HORIZONTAL).clone().id).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,108,114,101,113,45,104,111,110,103,107,111,110,103,45,104,111,114,105,122,111,110,116,97,108]), ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_HONG_KONG_HORIZONTAL).clone().id).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Mainland), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Taiwan), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::HongKong), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Custom), None).unwrap();
        let resolver = BuiltInClreqProfileResolver::new();
        let resolved_built_in = resolver.resolve((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_clreq_profile((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone(), (resolved_built_in).clone(), None).unwrap();
        let resolved_mainland_id = resolver.resolve(LayoutProfileId::new(&(UStr::new(&[99,108,114,101,113,45,109,97,105,110,108,97,110,100,45,104,111,114,105,122,111,110,116,97,108]))));
        let _ = TracedAssertions::traced_assertions_assert_equals_clreq_profile((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone(), (resolved_mainland_id).clone(), None).unwrap();
        let resolved_other_id = resolver.resolve(LayoutProfileId::new(&(UStr::new(&[111,116,104,101,114,45,112,114,111,102,105,108,101]))));
        let _ = TracedAssertions::traced_assertions_assert_equals_clreq_profile((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone(), (resolved_other_id).clone(), None).unwrap();
    });
}

#[test]
fn test_clreq_punctuation_policies_and_classification() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testClreqPunctuationPoliciesAndClassification", "org.tiqian.clreq.ClreqProfileCoverageTest.testClreqPunctuationPoliciesAndClassification", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,114,111,102,105,108,101,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,67,108,114,101,113,80,117,110,99,116,117,97,116,105,111,110,80,111,108,105,99,105,101,115,65,110,100,67,108,97,115,115,105,102,105,99,97,116,105,111,110]));
        let point_marks = vec![
    UString::from(",").to_ustring(),
    UString::from(".").to_ustring(),
    UString::from(":").to_ustring(),
    UString::from(";").to_ustring(),
    UString::from("!").to_ustring(),
    UString::from("?").to_ustring(),
];
        let mut point_index = 0u32;
        while (i32::from_ne_bytes(((point_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((point_marks.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((point_marks[usize::try_from(point_index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            point_index = u32::wrapping_add(point_index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark(UStr::new(&[97])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark(UStr::new(&[65292])), None).unwrap();
        let opening_chars = vec![
    UString::from("“").to_ustring(),
    UString::from("‘").to_ustring(),
    UString::from("（").to_ustring(),
    UString::from("《").to_ustring(),
    UString::from("〈").to_ustring(),
    UString::from("「").to_ustring(),
    UString::from("『").to_ustring(),
    UString::from("【").to_ustring(),
    UString::from("〔").to_ustring(),
    UString::from("〖").to_ustring(),
    UString::from("〘").to_ustring(),
    UString::from("〚").to_ustring(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((opening_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Opening, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((opening_chars[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let closing_chars = vec![
    UString::from("”").to_ustring(),
    UString::from("’").to_ustring(),
    UString::from("）").to_ustring(),
    UString::from("》").to_ustring(),
    UString::from("〉").to_ustring(),
    UString::from("」").to_ustring(),
    UString::from("』").to_ustring(),
    UString::from("】").to_ustring(),
    UString::from("〕").to_ustring(),
    UString::from("〗").to_ustring(),
    UString::from("〙").to_ustring(),
    UString::from("〛").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((closing_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Closing, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((closing_chars[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let pause_or_stop_chars = vec![
    UString::from("，").to_ustring(),
    UString::from("、").to_ustring(),
    UString::from("。").to_ustring(),
    UString::from("；").to_ustring(),
    UString::from("：").to_ustring(),
    UString::from("！").to_ustring(),
    UString::from("？").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((pause_or_stop_chars.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::PauseOrStop, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((pause_or_stop_chars[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::MiddleDot, ClreqPunctuationPolicies::clreq_punctuation_policies_classify(UStr::new(&[183])), None).unwrap();
        let interpuncts = vec![
    UString::from("・").to_ustring(),
    UString::from("‧").to_ustring(),
    UString::from("•").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((interpuncts.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Interpunct, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((interpuncts[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let connectors = vec![
    UString::from("～").to_ustring(),
    UString::from("~").to_ustring(),
    UString::from("-").to_ustring(),
    UString::from("–").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((connectors.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Connector, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((connectors[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let solidi = vec![UString::from("/").to_ustring(), UString::from("／").to_ustring()];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((solidi.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Solidus, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((solidi[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let ellipses = vec![UString::from("…").to_ustring(), UString::from("⋯").to_ustring()];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((ellipses.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Ellipsis, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((ellipses[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let dashes = vec![UString::from("—").to_ustring(), UString::from("⸺").to_ustring()];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((dashes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Dash, ClreqPunctuationPolicies::clreq_punctuation_policies_classify((dashes[usize::try_from(index).unwrap_or(0)]).clone().as_ustr()), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_class(PunctuationClass::Other, ClreqPunctuationPolicies::clreq_punctuation_policies_classify(UStr::new(&[20013])), None).unwrap();
    });
}

#[test]
fn test_forced_half_width_and_policy_for() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testForcedHalfWidthAndPolicyFor", "org.tiqian.clreq.ClreqProfileCoverageTest.testForcedHalfWidthAndPolicyFor", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,114,111,102,105,108,101,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,70,111,114,99,101,100,72,97,108,102,87,105,100,116,104,65,110,100,80,111,108,105,99,121,70,111,114]));
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[45]), PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[8211]), PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))), None).unwrap();
        let gb_policy = PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(true));
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65374]), (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[183]), (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[8226]), (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[47]), (gb_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65292]), (gb_policy).clone()), None).unwrap();
        let kaiming_policy = PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::Kaiming), Some(false));
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65288]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65289]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65292]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65307]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[12290]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65281]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65311]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[65294]), (kaiming_policy).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(UStr::new(&[20013]), (kaiming_policy).clone()), None).unwrap();
        let dash2_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[11834]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, dash2_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, dash2_policy.default_advance_em, None).unwrap();
        let hyphen_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[45]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, hyphen_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, hyphen_policy.default_advance_em, None).unwrap();
        let comma_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[65292]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, comma_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, comma_policy.default_advance_em, None).unwrap();
        let open_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[65288]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, open_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, open_policy.default_advance_em, None).unwrap();
        let close_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[65289]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.5f64, close_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, close_policy.default_advance_em, None).unwrap();
        let han_policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(UStr::new(&[23383]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, han_policy.default_body_em, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, han_policy.default_advance_em, None).unwrap();
    });
}

#[test]
fn test_forbidden_at_line_start_and_end() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testForbiddenAtLineStartAndEnd", "org.tiqian.clreq.ClreqProfileCoverageTest.testForbiddenAtLineStartAndEnd", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,114,111,102,105,108,101,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116,65,110,100,69,110,100]));
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65292]), KinsokuLevel::None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[65288]), KinsokuLevel::None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65292]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65289]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65374]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[183]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8226]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[47]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8212]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8212]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8230]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8230]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65288]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[23383]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[65288]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[47]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[47]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[65289]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[23383]), KinsokuLevel::Strict), None).unwrap();
    });
}

#[test]
fn test_punctuation_advance_and_substitutor() {
    testlib::run("org.tiqian.clreq.ClreqProfileCoverageTest.testPunctuationAdvanceAndSubstitutor", "org.tiqian.clreq.ClreqProfileCoverageTest.testPunctuationAdvanceAndSubstitutor", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,114,111,102,105,108,101,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,80,117,110,99,116,117,97,116,105,111,110,65,100,118,97,110,99,101,65,110,100,83,117,98,115,116,105,116,117,116,111,114]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(UStr::new(&[11834]), UStr::new(&[11834])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(UStr::new(&[8212]), UStr::new(&[11834])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(UStr::new(&[11834]), UStr::new(&[8212,8212])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(UStr::new(&[97,98,99]), UStr::new(&[97,98,99])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(UStr::new(&[55357,56832]), UStr::new(&[100,117,109,109,121])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_surrogate_text(&vec![55296]).as_ustr(), UStr::new(&[100,117,109,109,121])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_surrogate_text(&vec![55296, 65]).as_ustr(), UStr::new(&[100,117,109,109,121])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ClreqPunctuationAdvancePolicy::clreq_punctuation_advance_policy_advance_em(ClreqProfileCoverageTestHelpers::clreq_profile_coverage_test_helpers_surrogate_text(&vec![55296, 57344]).as_ustr(), UStr::new(&[100,117,109,109,121])), None).unwrap();
        let preserve_substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreserveInput));
        let preserve_res = preserve_substitutor.substitute(UStr::new(&[8230,8230])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8230,8230]), (preserve_res.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&((preserve_res.reason).to_ustring()), UString::from("preserve").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let prefer_substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints));
        let prefer_res = prefer_substitutor.substitute(UStr::new(&[8212,8212])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8212,8212]), (prefer_res.source_text).to_ustring().as_ustr(), None).unwrap();
        let force_substitutor = ClreqPunctuationGlyphSubstitutor::new(Some(CjkPunctuationGlyphPolicy::ForceClreqRecommendedCodepoints));
        let force_res = force_substitutor.substitute(UStr::new(&[97,98,99])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[97,98,99]), (force_res.display_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&((force_res.reason).to_ustring()), UString::from("preserve").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct ClreqProfileCoverageTestHelpers;

impl ClreqProfileCoverageTestHelpers {
    pub fn clreq_profile_coverage_test_helpers_surrogate_text(codes: &Vec<u32>) -> UString {
        let mut output = UString::new();
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            output += &(if codes[usize::try_from(index).unwrap_or(0)] > 0xFFFF { u_string::from_units(&[0xD800 + (((codes[usize::try_from(index).unwrap_or(0)]) - 0x10000) >> 10) as u16, 0xDC00 + (((codes[usize::try_from(index).unwrap_or(0)]) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(codes[usize::try_from(index).unwrap_or(0)]) as u16]) });
            index = u32::wrapping_add(index, 1);
        }
        return output;
    }

    pub fn clreq_profile_coverage_test_helpers_contains_int(values: &[u32], value: u32) -> bool {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if values[usize::try_from(index).unwrap_or(0)] == value {
                return true;
            }
            index = u32::wrapping_add(index, 1);
        }
        return false;
    }
}
