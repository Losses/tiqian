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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ClreqPolicyTailCoverageTestKinsokuRuleAllowsClustersWithoutDisplayTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,111,108,105,99,121,84,97,105,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[102,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116,67,111,118,101,114,115,69,118,101,114,121,80,117,110,99,116,117,97,116,105,111,110,67,108,97,115,115]));
        let levels = vec![KinsokuLevel::Basic, KinsokuLevel::GbStyle, KinsokuLevel::Strict];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((levels.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let level = levels[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65292]), level), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("comma at ")); __s += UString::from(level.name()).as_ustr(); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8221]), level), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("closing quote at ")); __s += UString::from(level.name()).as_ustr(); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[183]), level), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("middle dot at ")); __s += UString::from(level.name()).as_ustr(); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[12539]), level), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("interpunct at ")); __s += UString::from(level.name()).as_ustr(); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65374]), level), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("connector at ")); __s += UString::from(level.name()).as_ustr(); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[47]), level), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("solidus at ")); __s += UString::from(level.name()).as_ustr(); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8212]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8212]), KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8212]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8230]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[8230]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[25991]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(UStr::new(&[65292]), KinsokuLevel::None), None).unwrap();
    });
}

#[test]
fn forbidden_at_line_end_covers_opening_solidus_and_other() {
    testlib::run("org.tiqian.clreq.ClreqPolicyTailCoverageTest.forbiddenAtLineEndCoversOpeningSolidusAndOther", "org.tiqian.clreq.ClreqPolicyTailCoverageTest.forbiddenAtLineEndCoversOpeningSolidusAndOther", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,111,108,105,99,121,84,97,105,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[102,111,114,98,105,100,100,101,110,65,116,76,105,110,101,69,110,100,67,111,118,101,114,115,79,112,101,110,105,110,103,83,111,108,105,100,117,115,65,110,100,79,116,104,101,114]));
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[8220]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[47]), KinsokuLevel::Basic), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[47]), KinsokuLevel::GbStyle), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[47]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[65292]), KinsokuLevel::Strict), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(UStr::new(&[8220]), KinsokuLevel::None), None).unwrap();
    });
}

#[test]
fn kinsoku_rule_allows_clusters_without_display_text() {
    testlib::run("org.tiqian.clreq.ClreqPolicyTailCoverageTest.kinsokuRuleAllowsClustersWithoutDisplayText", "org.tiqian.clreq.ClreqPolicyTailCoverageTest.kinsokuRuleAllowsClustersWithoutDisplayText", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,111,108,105,99,121,84,97,105,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[107,105,110,115,111,107,117,82,117,108,101,65,108,108,111,119,115,67,108,117,115,116,101,114,115,87,105,116,104,111,117,116,68,105,115,112,108,97,121,84,101,120,116]));
        let empty = Cluster::new(TextRange::new(0u32, 0u32).unwrap(), &(UStr::new(&[])), &(UStr::new(&[115,116,117,98])), 0.0f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0));
        let rule = ClreqKinsokuRule::new(Some(KinsokuLevel::Basic));
        let _ = TracedAssertions::traced_assertions_assert_false(rule.forbidden_at_line_start((empty).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(rule.forbidden_at_line_end((empty).clone()), None).unwrap();
    });
}

#[test]
fn bopomofo_parser_covers_every_tone_arm() {
    testlib::run("org.tiqian.clreq.ClreqPolicyTailCoverageTest.bopomofoParserCoversEveryToneArm", "org.tiqian.clreq.ClreqPolicyTailCoverageTest.bopomofoParserCoversEveryToneArm", || {
        TestTraceRecorder::new(&(UStr::new(&[67,108,114,101,113,80,111,108,105,99,121,84,97,105,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[98,111,112,111,109,111,102,111,80,97,114,115,101,114,67,111,118,101,114,115,69,118,101,114,121,84,111,110,101,65,114,109]));
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570])).tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄅ").to_ustring(), UString::from("ㄚ").to_ustring()], &BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570])).symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yangping, BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,714])).tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Shang, BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,711])).tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Qu, BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,715])).tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄅ").to_ustring(), UString::from("ㄚ").to_ustring()], &BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,715])).symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,713])).tone, None).unwrap();
        let neutral = BopomofoParser::bopomofo_parser_parse(UStr::new(&[729,12549,12570]));
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Neutral, neutral.tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄅ").to_ustring(), UString::from("ㄚ").to_ustring()], &neutral.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(UStr::new(&[])).tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,712])).tone, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![
    UString::from("ㄅ").to_ustring(),
    UString::from("ㄚ").to_ustring(),
    UString::from("ˈ").to_ustring(),
], &BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,712])).symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, BopomofoParser::bopomofo_parser_parse(UStr::new(&[12549,12570,716])).tone, None).unwrap();
    });
}
