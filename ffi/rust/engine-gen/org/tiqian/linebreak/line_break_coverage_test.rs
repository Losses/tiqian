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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for LineBreakCoverageTestTestUnicodePunctuationLineBreakFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakCoverageTestTestUnicodePunctuationLineBreakFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for LineBreakCoverageTestTestLineBreakModelsAndEnumsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakCoverageTestTestLineBreakModelsAndEnumsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for LineBreakCoverageTestTestBundledHyphenationResourceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakCoverageTestTestBundledHyphenationResourceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakCoverageTestTestBundledHyphenationResourceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,66,117,110,100,108,101,100,72,121,112,104,101,110,97,116,105,111,110,82,101,115,111,117,114,99,101]));
        let p = EnglishHyphenationPatterns::english_hyphenation_patterns_load().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(p))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("\\patterns").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_line_break_models_and_enums() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testLineBreakModelsAndEnums", "org.tiqian.linebreak.LineBreakCoverageTest.testLineBreakModelsAndEnums", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,76,105,110,101,66,114,101,97,107,77,111,100,101,108,115,65,110,100,69,110,117,109,115]));
        let o = BreakOpportunity::new(5u32, BreakKind::Allowed, &(UStr::new(&[84,101,115,116,82,101,97,115,111,110])), Some(10));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, o.index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(o.kind.name()).as_ustr(), UString::from(BreakKind::Allowed.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(10, o.penalty, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[84,101,115,116,82,101,97,115,111,110]), (o.reason).to_ustring().as_ustr(), None).unwrap();
        let c = BreakOpportunity::new(5u32, BreakKind::Problematic, &(UStr::new(&[84,101,115,116,82,101,97,115,111,110])), Some(10));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(c.kind.name()).as_ustr(), UString::from(BreakKind::Problematic.name()).as_ustr(), None).unwrap();
        let oc = BreakOpportunity::new(5u32, BreakKind::Allowed, &(UStr::new(&[84,101,115,116,82,101,97,115,111,110])), Some(10));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", o.to_string()).as_str()).as_ustr(), UString::from(format!("{}", oc.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(o.to_string() == oc.to_string(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", o.to_string()).as_str()), UString::from("BreakOpportunity").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let mut i = 0u32;
        let kinds = vec![BreakKind::Allowed, BreakKind::Forbidden, BreakKind::Required, BreakKind::Problematic];
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((kinds.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(kinds[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let f = ForbiddenBreak::new(TextRange::new(2u32, 6u32).unwrap(), &(UStr::new(&[70,111,114,98,105,100,100,101,110,82,101,97,115,111,110])));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", (f.range).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", TextRange::new(2u32, 6u32).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[70,111,114,98,105,100,100,101,110,82,101,97,115,111,110]), (f.reason).to_ustring().as_ustr(), None).unwrap();
        let fc = ForbiddenBreak::new(TextRange::new(2u32, 6u32).unwrap(), &(UStr::new(&[70,111,114,98,105,100,100,101,110,82,101,97,115,111,110])));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", f.to_string()).as_str()).as_ustr(), UString::from(format!("{}", fc.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(f.to_string() == fc.to_string(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", f.to_string()).as_str()), UString::from("ForbiddenBreak").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_mandatory_break_and_zero_width_space_code_points() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testMandatoryBreakAndZeroWidthSpaceCodePoints", "org.tiqian.linebreak.LineBreakCoverageTest.testMandatoryBreakAndZeroWidthSpaceCodePoints", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,77,97,110,100,97,116,111,114,121,66,114,101,97,107,65,110,100,90,101,114,111,87,105,100,116,104,83,112,97,99,101,67,111,100,101,80,111,105,110,116,115]));
        let yes = vec![10, 11, 12, 13, 133, 8232, 8233];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((yes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(LineBreakFns::line_break_fns_is_mandatory_break_code_point(yes[usize::try_from(i).unwrap_or(0)]), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Code point ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(yes[usize::try_from(i).unwrap_or(0)])).as_str())); __s += &(UString::from(" should be mandatory break")); __s }).as_str()))).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let no = vec![32, 65, 0, 8203, 8234];
        i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((no.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_mandatory_break_code_point(no[usize::try_from(i).unwrap_or(0)]), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Code point ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(no[usize::try_from(i).unwrap_or(0)])).as_str())); __s += &(UString::from(" should not be mandatory break")); __s }).as_str()))).unwrap();
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
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,83,105,109,112,108,101,67,104,97,114,97,99,116,101,114,76,105,110,101,66,114,101,97,107,65,110,97,108,121,122,101,114]));
        let a = SimpleCharacterLineBreakAnalyzer::new();
        let e = a.analyze(UStr::new(&[]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,93]), LineBreakCoverageTestHelpers::line_break_coverage_test_helpers_render_breaks(&e).as_ustr(), None).unwrap();
        let s = a.analyze(UStr::new(&[65]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, s[0usize].index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(s[0usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[83,105,109,112,108,101,67,104,97,114,97,99,116,101,114,76,105,110,101,66,114,101,97,107,65,110,97,108,121,122,101,114]), ((s[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let m = a.analyze(UStr::new(&[97,98,99]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((m.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(m[0usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(m[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(m[2usize].kind.name()).as_ustr(), None).unwrap();
        let l = a.analyze(UStr::new(&[97,10,98]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((l.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(l[0usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(l[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107]), ((l[1usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(l[2usize].kind.name()).as_ustr(), None).unwrap();
        let cr = a.analyze(UStr::new(&[97,13,10,98]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((cr.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(cr[0usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(cr[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[83,105,109,112,108,101,67,104,97,114,97,99,116,101,114,76,105,110,101,66,114,101,97,107,65,110,97,108,121,122,101,114]), ((cr[1usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(cr[2usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107]), ((cr[2usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(cr[3usize].kind.name()).as_ustr(), None).unwrap();
        let co = a.analyze(UStr::new(&[97,13,98]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((co.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(co[0usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(co[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107]), ((co[1usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(co[2usize].kind.name()).as_ustr(), None).unwrap();
        let ce = a.analyze(UStr::new(&[97,13]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((ce.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(ce[0usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(ce[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107]), ((ce[1usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn test_hyphenation_components() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testHyphenationComponents", "org.tiqian.linebreak.LineBreakCoverageTest.testHyphenationComponents", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,72,121,112,104,101,110,97,116,105,111,110,67,111,109,112,111,110,101,110,116,115]));
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![UString::from("hyp").to_ustring(), UString::from("phen").to_ustring()], &vec![(vec![0, 0, 1, 0]).clone(), (vec![0, 0, 2, 0, 0]).clone()]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![UString::from("specialword").to_ustring()], &vec![(vec![1, 4, 10]).clone()])), Some(2), Some(3));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &NoHyphenator::new().hyphenate(UStr::new(&[119,111,114,100])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(UStr::new(&[116,101,115,116])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![4], &h.hyphenate(UStr::new(&[83,112,101,99,105,97,108,87,111,114,100])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(UStr::new(&[122,122,122,122,122,122])), None).unwrap();
        let e = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(UStr::new(&[37,32,111,110,108,121,32,99,111,109,109,101,110,116,115,10,32,32,32]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes(((u32::from_ne_bytes(((e.patterns.size()) as u32).to_ne_bytes())) as u32).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes(((u32::from_ne_bytes(((e.exceptions.size()) as u32).to_ne_bytes())) as u32).to_ne_bytes()), None).unwrap();
        let m = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(UStr::new(&[92,112,97,116,116,101,114,110,115,32,110,111,32,98,114,97,99,101,115,32,92,104,121,112,104,101,110,97,116,105,111,110,32,110,111,32,98,114,97,99,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes(((u32::from_ne_bytes(((m.patterns.size()) as u32).to_ne_bytes())) as u32).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes(((u32::from_ne_bytes(((m.exceptions.size()) as u32).to_ne_bytes())) as u32).to_ne_bytes()), None).unwrap();
        let u = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(UStr::new(&[92,112,97,116,116,101,114,110,115,32,123,32,97,98,99,32,10,92,104,121,112,104,101,110,97,116,105,111,110,32,123,32,100,101,102]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes(((u32::from_ne_bytes(((u.patterns.size()) as u32).to_ne_bytes())) as u32).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::from_ne_bytes(((u32::from_ne_bytes(((u.exceptions.size()) as u32).to_ne_bytes())) as u32).to_ne_bytes()), None).unwrap();
        let v = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(UStr::new(&[92,112,97,116,116,101,114,110,115,123,32,46,97,98,51,99,100,46,32,101,49,102,32,125,92,104,121,112,104,101,110,97,116,105,111,110,123,32,97,115,45,115,111,45,99,105,97,116,101,32,100,105,115,45,97,108,108,111,119,45,32,125]));
        let _ = TracedAssertions::traced_assertions_assert_true(v.patterns.get(&(UString::from(".abcd.")).to_ustring()).is_some(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(v.exceptions.get(&(UString::from("associate")).to_ustring()).is_some(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2, 4], &(v.exceptions.get(&(UString::from("associate")).to_ustring())).as_ref().unwrap(), None).unwrap();
    });
}

#[test]
fn test_unicode_punctuation_line_break() {
    testlib::run("org.tiqian.linebreak.LineBreakCoverageTest.testUnicodePunctuationLineBreak", "org.tiqian.linebreak.LineBreakCoverageTest.testUnicodePunctuationLineBreak", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,76,105,110,101,66,114,101,97,107]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[49,55,46,48,46,48]), UStr::new(&[49,55,46,48,46,48]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UString::from("https://www.unicode.org/Public/17.0.0/ucd/LineBreak.txt")))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UString::from("e6a18fa91f8f6a6f8e534b1d3f128c21ada45bfe152eb6b1bcc5e15fd8ac92e6")))) as i32).to_ne_bytes())) > (0), None).unwrap();
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
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((kinds.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(kinds[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), None).unwrap();
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
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((cps.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(kinds[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(cps[usize::try_from(i).unwrap_or(0)]).unwrap().name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[derive(Clone, Copy)]
pub struct LineBreakCoverageTestHelpers;

impl LineBreakCoverageTestHelpers {
    pub fn line_break_coverage_test_helpers_render_breaks(values: &[BreakOpportunity]) -> UString {
        let mut b_b = UString::new();
        b_b += &(UString::from("["));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                b_b += &(UString::from(", "));
            }
            {
                let x = UString::from(format!("{}", (values[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str());
                b_b += &(x.to_string());
            }
            i = u32::wrapping_add(i, 1);
        }
        b_b += &(UString::from("]"));
        return b_b;
    }
}
