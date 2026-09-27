#![cfg(test)]

use crate::org::tiqian::linebreak::line_break_analyzer::SimpleCharacterLineBreakAnalyzer;
use crate::org::tiqian::linebreak::line_break_fns::LineBreakFns;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn recognizes_mandatory_break_code_points() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.recognizesMandatoryBreakCodePoints", "org.tiqian.linebreak.MandatoryBreakTest.recognizesMandatoryBreakCodePoints", || {
        TestTraceRecorder::new(&(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[114,101,99,111,103,110,105,122,101,115,77,97,110,100,97,116,111,114,121,66,114,101,97,107,67,111,100,101,80,111,105,110,116,115]));
        let y = vec![10, 11, 12, 13, 133, 8232, 8233];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((y.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(LineBreakFns::line_break_fns_is_mandatory_break_code_point(y[usize::try_from(i).unwrap_or(0)]), Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(y[usize::try_from(i).unwrap_or(0)])).to_ustring())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let n = vec![97, 20013, 32, 9, 12288];
        i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((n.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_mandatory_break_code_point(n[usize::try_from(i).unwrap_or(0)]), Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(n[usize::try_from(i).unwrap_or(0)])).to_ustring())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn recognizes_zero_width_space_without_conflating_no_break_controls() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.recognizesZeroWidthSpaceWithoutConflatingNoBreakControls", "org.tiqian.linebreak.MandatoryBreakTest.recognizesZeroWidthSpaceWithoutConflatingNoBreakControls", || {
        TestTraceRecorder::new(&(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[114,101,99,111,103,110,105,122,101,115,90,101,114,111,87,105,100,116,104,83,112,97,99,101,87,105,116,104,111,117,116,67,111,110,102,108,97,116,105,110,103,78,111,66,114,101,97,107,67,111,110,116,114,111,108,115]));
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakFns::line_break_fns_is_zero_width_space_code_point(8203), Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(8203)).to_ustring())).unwrap();
        let n = vec![8204, 8205, 8288, 65279];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((n.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_zero_width_space_code_point(n[usize::try_from(i).unwrap_or(0)]), Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(n[usize::try_from(i).unwrap_or(0)])).to_ustring())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn marks_required_after_line_feed() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.marksRequiredAfterLineFeed", "org.tiqian.linebreak.MandatoryBreakTest.marksRequiredAfterLineFeed", || {
        TestTraceRecorder::new(&(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[109,97,114,107,115,82,101,113,117,105,114,101,100,65,102,116,101,114,76,105,110,101,70,101,101,100]));
        let o = SimpleCharacterLineBreakAnalyzer::new().analyze(UStr::new(&[97,10,98]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(o[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(o[0usize].kind.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn collapses_crlf_to_a_single_break_after_lf() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.collapsesCrlfToASingleBreakAfterLf", "org.tiqian.linebreak.MandatoryBreakTest.collapsesCrlfToASingleBreakAfterLf", || {
        TestTraceRecorder::new(&(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[99,111,108,108,97,112,115,101,115,67,114,108,102,84,111,65,83,105,110,103,108,101,66,114,101,97,107,65,102,116,101,114,76,102]));
        let o = SimpleCharacterLineBreakAnalyzer::new().analyze(UStr::new(&[97,13,10,98]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,108,111,119,101,100]), UString::from(o[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(o[2usize].kind.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn preserves_each_blank_line_break() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.preservesEachBlankLineBreak", "org.tiqian.linebreak.MandatoryBreakTest.preservesEachBlankLineBreak", || {
        TestTraceRecorder::new(&(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[112,114,101,115,101,114,118,101,115,69,97,99,104,66,108,97,110,107,76,105,110,101,66,114,101,97,107]));
        let o = SimpleCharacterLineBreakAnalyzer::new().analyze(UStr::new(&[97,10,10,98]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(o[1usize].kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,101,113,117,105,114,101,100]), UString::from(o[2usize].kind.name()).as_ustr(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct MandatoryBreakTestHelpers;

impl MandatoryBreakTestHelpers {
    pub fn mandatory_break_test_helpers_u(cp: u32) -> UString {
        let s = UString::from(format!("{}", format!("{:0w$X}", cp, w = usize::try_from(4).unwrap_or_default())).as_str());
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("U+")); __s += s.as_ustr(); __s }).as_str());
    }
}
