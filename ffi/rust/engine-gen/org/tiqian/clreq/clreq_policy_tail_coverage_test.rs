#![cfg(test)]

use crate::org::tiqian::clreq::bopomofo_parser::BopomofoParser;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault) -> Self {
        match value {
            ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault) -> Self {
        match value {
            ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault) -> Self {
        match value {
            ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn forbidden_at_line_start_covers_every_punctuation_class() {
    testlib::run("org.tiqian.clreq.ClreqPolicyTailCoverageTest.forbiddenAtLineStartCoversEveryPunctuationClass", "org.tiqian.clreq.ClreqPolicyTailCoverageTest.forbiddenAtLineStartCoversEveryPunctuationClass", || {
        TestTraceRecorder::new("ClreqPolicyTailCoverageTest").section(&"forbiddenAtLineStartCoversEveryPunctuationClass");
        let levels = vec![KinsokuLevel::Basic, KinsokuLevel::GbStyle, KinsokuLevel::Strict];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((levels.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let level = levels[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"，", level), Some((format!("{}{}",
            "comma at ",
            level.name()
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"”", level), Some((format!("{}{}",
            "closing quote at ",
            level.name()
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"·", level), Some((format!("{}{}",
            "middle dot at ",
            level.name()
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"・", level), Some((format!("{}{}",
            "interpunct at ",
            level.name()
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"～", level), Some((format!("{}{}",
            "connector at ",
            level.name()
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"/", level), Some((format!("{}{}",
            "solidus at ",
            level.name()
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"—", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"—", KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"—", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"…", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"…", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"文", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(&"，", KinsokuLevel::None), None).unwrap();
    });
}

#[test]
fn forbidden_at_line_end_covers_opening_solidus_and_other() {
    testlib::run("org.tiqian.clreq.ClreqPolicyTailCoverageTest.forbiddenAtLineEndCoversOpeningSolidusAndOther", "org.tiqian.clreq.ClreqPolicyTailCoverageTest.forbiddenAtLineEndCoversOpeningSolidusAndOther", || {
        TestTraceRecorder::new("ClreqPolicyTailCoverageTest").section(&"forbiddenAtLineEndCoversOpeningSolidusAndOther");
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"“", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"/", KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"/", KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"/", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"，", KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(&"“", KinsokuLevel::None), None).unwrap();
    });
}

#[test]
fn kinsoku_rule_allows_clusters_without_display_text() {
    testlib::run("org.tiqian.clreq.ClreqPolicyTailCoverageTest.kinsokuRuleAllowsClustersWithoutDisplayText", "org.tiqian.clreq.ClreqPolicyTailCoverageTest.kinsokuRuleAllowsClustersWithoutDisplayText", || {
        TestTraceRecorder::new("ClreqPolicyTailCoverageTest").section(&"kinsokuRuleAllowsClustersWithoutDisplayText");
        let empty = Cluster::new(TextRange::new(0u32, 0u32).unwrap(), "", "stub", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0));
        let rule = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let _ = TracedAssertions::traced_assertions_assert_false(rule.forbidden_at_line_start((empty).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(rule.forbidden_at_line_end((empty).clone()), None).unwrap();
    });
}

#[test]
fn bopomofo_parser_covers_every_tone_arm() {
    testlib::run("org.tiqian.clreq.ClreqPolicyTailCoverageTest.bopomofoParserCoversEveryToneArm", "org.tiqian.clreq.ClreqPolicyTailCoverageTest.bopomofoParserCoversEveryToneArm", || {
        TestTraceRecorder::new("ClreqPolicyTailCoverageTest").section(&"bopomofoParserCoversEveryToneArm");
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(&"ㄅㄚ").tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄅ".to_string(), "ㄚ".to_string()], &BopomofoParser::bopomofo_parser_parse(&"ㄅㄚ").symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yangping, BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˊ").tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Shang, BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˇ").tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Qu, BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˋ").tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄅ".to_string(), "ㄚ".to_string()], &BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˋ").symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˉ").tone, None).unwrap();
        let neutral = BopomofoParser::bopomofo_parser_parse(&"˙ㄅㄚ");
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Neutral, neutral.tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄅ".to_string(), "ㄚ".to_string()], &neutral.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(&"").tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˈ").tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄅ".to_string(), "ㄚ".to_string(), "ˈ".to_string()], &BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˈ").symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(&"ㄅㄚˌ").tone, None).unwrap();
    });
}
