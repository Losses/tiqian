#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::linebreak::break_kind::BreakKind;
use crate::org::tiqian::linebreak::break_opportunity::BreakOpportunity;
use crate::org::tiqian::linebreak::break_opportunity::ForbiddenBreak;
use crate::org::tiqian::linebreak::english_hyphenation_patterns::EnglishHyphenationPatterns;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::linebreak::liang_hyphenator::LiangHyphenator;
use crate::org::tiqian::linebreak::liang_hyphenator_test::LiangHyphenatorTestHelpers;
use crate::org::tiqian::linebreak::line_break_analyzer::SimpleCharacterLineBreakAnalyzer;
use crate::org::tiqian::linebreak::line_break_fns::LineBreakFns;
use crate::org::tiqian::linebreak::parse_tex_hyphenation_patterns::ParseTexHyphenationPatterns;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreak;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreakClass;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<LineBreakCoverageTestTestUnicodePunctuationLineBreakFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakCoverageTestTestUnicodePunctuationLineBreakFault) -> Self {
        match value {
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakCoverageTestTestUnicodePunctuationLineBreakFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineBreakCoverageTestTestUnicodePunctuationLineBreakFault) -> Self {
        match value {
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakCoverageTestTestUnicodePunctuationLineBreakFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakCoverageTestTestUnicodePunctuationLineBreakFault) -> Self {
        match value {
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakCoverageTestTestUnicodePunctuationLineBreakFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakCoverageTestTestUnicodePunctuationLineBreakFault) -> Self {
        match value {
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakCoverageTestTestUnicodePunctuationLineBreakFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineBreakCoverageTestTestUnicodePunctuationLineBreakFault) -> Self {
        match value {
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakCoverageTestTestLineBreakModelsAndEnumsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakCoverageTestTestLineBreakModelsAndEnumsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakCoverageTestTestLineBreakModelsAndEnumsFault) -> Self {
        match value {
            LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakCoverageTestTestLineBreakModelsAndEnumsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakCoverageTestTestLineBreakModelsAndEnumsFault) -> Self {
        match value {
            LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakCoverageTestTestLineBreakModelsAndEnumsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakCoverageTestTestLineBreakModelsAndEnumsFault) -> Self {
        match value {
            LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakCoverageTestTestLineBreakModelsAndEnumsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakCoverageTestTestLineBreakModelsAndEnumsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakCoverageTestTestLineBreakModelsAndEnumsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakCoverageTestTestBundledHyphenationResourceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakCoverageTestTestBundledHyphenationResourceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakCoverageTestTestBundledHyphenationResourceFault) -> Self {
        match value {
            LineBreakCoverageTestTestBundledHyphenationResourceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakCoverageTestTestBundledHyphenationResourceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakCoverageTestTestBundledHyphenationResourceFault) -> Self {
        match value {
            LineBreakCoverageTestTestBundledHyphenationResourceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakCoverageTestTestBundledHyphenationResourceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakCoverageTestTestBundledHyphenationResourceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakCoverageTestTestBundledHyphenationResourceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakCoverageTestTestBundledHyphenationResourceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn test_bundled_hyphenation_resource() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testBundledHyphenationResource", "org.tiqian.linebreak.LineBreakCoverageTest.testBundledHyphenationResource", || {
        TestTraceRecorder::new("LineBreakCoverageTest").section(&"testBundledHyphenationResource");
        let p = EnglishHyphenationPatterns::english_hyphenation_patterns_load().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(p))).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&p, "\\patterns", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_line_break_models_and_enums() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testLineBreakModelsAndEnums", "org.tiqian.linebreak.LineBreakCoverageTest.testLineBreakModelsAndEnums", || {
        TestTraceRecorder::new("LineBreakCoverageTest").section(&"testLineBreakModelsAndEnums");
        let o = BreakOpportunity::new(5u32, BreakKind::Allowed, "TestReason", Some(10));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, o.index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(o.kind.name().to_string().as_str(), BreakKind::Allowed.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(10, o.penalty, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"TestReason", (o.reason).to_string().as_str(), None).unwrap();
        let c = BreakOpportunity::new(5u32, BreakKind::Problematic, "TestReason", Some(10));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(c.kind.name().to_string().as_str(), BreakKind::Problematic.name().to_string().as_str(), None).unwrap();
        let oc = BreakOpportunity::new(5u32, BreakKind::Allowed, "TestReason", Some(10));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(o.to_string().as_str(), oc.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(o.to_string() == oc.to_string(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&o.to_string(), "BreakOpportunity", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let mut i = 0u32;
        let kinds = vec![BreakKind::Allowed, BreakKind::Forbidden, BreakKind::Required, BreakKind::Problematic];
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((kinds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(kinds[usize::try_from(i).unwrap_or(0)].name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let f = ForbiddenBreak::new(TextRange::new(2u32, 6u32).unwrap(), "ForbiddenReason");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered((f.range).clone().to_string().as_str(), TextRange::new(2u32, 6u32).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ForbiddenReason", (f.reason).to_string().as_str(), None).unwrap();
        let fc = ForbiddenBreak::new(TextRange::new(2u32, 6u32).unwrap(), "ForbiddenReason");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(f.to_string().as_str(), fc.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(f.to_string() == fc.to_string(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&f.to_string(), "ForbiddenBreak", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_mandatory_break_and_zero_width_space_code_points() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testMandatoryBreakAndZeroWidthSpaceCodePoints", "org.tiqian.linebreak.LineBreakCoverageTest.testMandatoryBreakAndZeroWidthSpaceCodePoints", || {
        TestTraceRecorder::new("LineBreakCoverageTest").section(&"testMandatoryBreakAndZeroWidthSpaceCodePoints");
        let yes = vec![10, 11, 12, 13, 133, 8232, 8233];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((yes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(LineBreakFns::line_break_fns_is_mandatory_break_code_point(yes[usize::try_from(i).unwrap_or(0)]), Some((format!("{}{}{}",
            "Code point ",
            crate::runtime::int_text::IntText::int_text(yes[usize::try_from(i).unwrap_or(0)]),
            " should be mandatory break"
        )).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let no = vec![32, 65, 0, 8203, 8234];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((no.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_mandatory_break_code_point(no[usize::try_from(i).unwrap_or(0)]), Some((format!("{}{}{}",
            "Code point ",
            crate::runtime::int_text::IntText::int_text(no[usize::try_from(i).unwrap_or(0)]),
            " should not be mandatory break"
        )).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakFns::line_break_fns_is_zero_width_space_code_point(8203), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_zero_width_space_code_point(8204), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_zero_width_space_code_point(8288), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_zero_width_space_code_point(65279), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_zero_width_space_code_point(32), None).unwrap();
    });
}

#[test]
fn test_simple_character_line_break_analyzer() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testSimpleCharacterLineBreakAnalyzer", "org.tiqian.linebreak.LineBreakCoverageTest.testSimpleCharacterLineBreakAnalyzer", || {
        TestTraceRecorder::new("LineBreakCoverageTest").section(&"testSimpleCharacterLineBreakAnalyzer");
        let a = SimpleCharacterLineBreakAnalyzer::new();
        let e = a.analyze(&"");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", LineBreakCoverageTestHelpers::line_break_coverage_test_helpers_render_breaks(&e).as_str(), None).unwrap();
        let s = a.analyze(&"A");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, s[0usize].index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", s[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"SimpleCharacterLineBreakAnalyzer", ((s[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let m = a.analyze(&"abc");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((m.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", m[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", m[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", m[2usize].kind.name().to_string().as_str(), None).unwrap();
        let l = a.analyze(&concat!("a\n",
"b"));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((l.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", l[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", l[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"MandatoryBreak", ((l[1usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", l[2usize].kind.name().to_string().as_str(), None).unwrap();
        let cr = a.analyze(&concat!("a\r\n",
"b"));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((cr.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", cr[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", cr[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"SimpleCharacterLineBreakAnalyzer", ((cr[1usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", cr[2usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"MandatoryBreak", ((cr[2usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", cr[3usize].kind.name().to_string().as_str(), None).unwrap();
        let co = a.analyze(&"a\rb");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((co.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", co[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", co[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"MandatoryBreak", ((co[1usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", co[2usize].kind.name().to_string().as_str(), None).unwrap();
        let ce = a.analyze(&"a\r");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((ce.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", ce[0usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", ce[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"MandatoryBreak", ((ce[1usize]).clone().reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn test_hyphenation_components() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testHyphenationComponents", "org.tiqian.linebreak.LineBreakCoverageTest.testHyphenationComponents", || {
        TestTraceRecorder::new("LineBreakCoverageTest").section(&"testHyphenationComponents");
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec!["hyp".to_string(), "phen".to_string()], &vec![(vec![0, 0, 1, 0]).clone(), (vec![0, 0, 2, 0, 0]).clone()]),
Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec!["specialword".to_string()], &vec![(vec![1, 4, 10]).clone()])), Some(2), Some(3));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &NoHyphenator::new().hyphenate(&"word"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(&"test"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![4], &h.hyphenate(&"SpecialWord"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(&"zzzzzz"), None).unwrap();
        let e = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(&concat!("% only comments\n",
"   "));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes((u32::from_ne_bytes((e.patterns.size()).to_ne_bytes())).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes((u32::from_ne_bytes((e.exceptions.size()).to_ne_bytes())).to_ne_bytes()), None).unwrap();
        let m = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(&"\\patterns no braces \\hyphenation no braces");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes((u32::from_ne_bytes((m.patterns.size()).to_ne_bytes())).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes((u32::from_ne_bytes((m.exceptions.size()).to_ne_bytes())).to_ne_bytes()), None).unwrap();
        let u = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(&concat!("\\patterns { abc \n",
"\\hyphenation { def"));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes((u32::from_ne_bytes((u.patterns.size()).to_ne_bytes())).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes((u32::from_ne_bytes((u.exceptions.size()).to_ne_bytes())).to_ne_bytes()), None).unwrap();
        let v = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(&"\\patterns{ .ab3cd. e1f }\\hyphenation{ as-so-ciate dis-allow- }");
        let _ = TracedAssertions::traced_assertions_assert_true(v.patterns.get(&(".abcd.").to_string()).is_some(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(v.exceptions.get(&("associate").to_string()).is_some(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2, 4], &(v.exceptions.get(&("associate").to_string())).as_ref().unwrap(), None).unwrap();
    });
}

#[test]
fn test_unicode_punctuation_line_break() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testUnicodePunctuationLineBreak", "org.tiqian.linebreak.LineBreakCoverageTest.testUnicodePunctuationLineBreak", || {
        TestTraceRecorder::new("LineBreakCoverageTest").section(&"testUnicodePunctuationLineBreak");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"17.0.0", &"17.0.0", None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&("https://www.unicode.org/Public/17.0.0/ucd/LineBreak.txt"))).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&("e6a18fa91f8f6a6f8e534b1d3f128c21ada45bfe152eb6b1bcc5e15fd8ac92e6"))).to_ne_bytes())) > (0), None).unwrap();
        let mut i = 0u32;
        let kinds = vec![
    UnicodePunctuationLineBreakClass::BreakAfter,
    UnicodePunctuationLineBreakClass::BreakBoth,
    UnicodePunctuationLineBreakClass::ClosePunctuation,
    UnicodePunctuationLineBreakClass::CloseParenthesis,
    UnicodePunctuationLineBreakClass::Exclamation,
    UnicodePunctuationLineBreakClass::HyphenHh,
    UnicodePunctuationLineBreakClass::Hyphen,
    UnicodePunctuationLineBreakClass::Inseparable,
    UnicodePunctuationLineBreakClass::InfixNumericSeparator,
    UnicodePunctuationLineBreakClass::Nonstarter,
    UnicodePunctuationLineBreakClass::OpenPunctuation,
    UnicodePunctuationLineBreakClass::Quotation,
    UnicodePunctuationLineBreakClass::SymbolsAllowingBreakAfter,
    UnicodePunctuationLineBreakClass::Other,
];
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((kinds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(kinds[usize::try_from(i).unwrap_or(0)].name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(4294967295u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(1114112).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(55296).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(57343).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let cps = vec![9, 8212, 125, 41, 33, 1418, 45, 8229, 44, 12293, 40, 34, 47, 65];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((cps.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(kinds[usize::try_from(i).unwrap_or(0)].name().to_string().as_str(),
UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(cps[usize::try_from(i).unwrap_or(0)]).unwrap().name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[derive(Clone, Copy)]
pub struct LineBreakCoverageTestHelpers;

impl LineBreakCoverageTestHelpers {
    pub fn line_break_coverage_test_helpers_render_breaks(values: &[BreakOpportunity]) -> String {
        let mut b_b = String::new();
        b_b += &("[");
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                b_b += &(", ");
            }
            {
                let x = (values[usize::try_from(i).unwrap_or(0)]).clone().to_string();
                b_b += &(x.to_string());
            }
            i = u32::wrapping_add(i, 1);
        }
        b_b += &("]");
        return b_b;
    }
}
