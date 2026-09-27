#![cfg(test)]

use crate::org::tiqian::linebreak::line_break_analyzer::SimpleCharacterLineBreakAnalyzer;
use crate::org::tiqian::linebreak::line_break_fns::LineBreakFns;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn recognizes_mandatory_break_code_points() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.recognizesMandatoryBreakCodePoints", "org.tiqian.linebreak.MandatoryBreakTest.recognizesMandatoryBreakCodePoints", || {
        TestTraceRecorder::new("MandatoryBreakTest").section(&"recognizesMandatoryBreakCodePoints");
        let y = vec![10, 11, 12, 13, 133, 8232, 8233];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((y.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_true(LineBreakFns::line_break_fns_is_mandatory_break_code_point(y[usize::try_from(i).unwrap_or(0)]),
Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(y[usize::try_from(i).unwrap_or(0)])).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let n = vec![97, 20013, 32, 9, 12288];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((n.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_mandatory_break_code_point(n[usize::try_from(i).unwrap_or(0)]),
Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(n[usize::try_from(i).unwrap_or(0)])).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn recognizes_zero_width_space_without_conflating_no_break_controls() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.recognizesZeroWidthSpaceWithoutConflatingNoBreakControls", "org.tiqian.linebreak.MandatoryBreakTest.recognizesZeroWidthSpaceWithoutConflatingNoBreakControls", || {
        TestTraceRecorder::new("MandatoryBreakTest").section(&"recognizesZeroWidthSpaceWithoutConflatingNoBreakControls");
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakFns::line_break_fns_is_zero_width_space_code_point(8203), Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(8203)).to_string())).unwrap();
        let n = vec![8204, 8205, 8288, 65279];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((n.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let _ = TracedAssertions::traced_assertions_assert_false(LineBreakFns::line_break_fns_is_zero_width_space_code_point(n[usize::try_from(i).unwrap_or(0)]),
Some((MandatoryBreakTestHelpers::mandatory_break_test_helpers_u(n[usize::try_from(i).unwrap_or(0)])).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn marks_required_after_line_feed() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.marksRequiredAfterLineFeed", "org.tiqian.linebreak.MandatoryBreakTest.marksRequiredAfterLineFeed", || {
        TestTraceRecorder::new("MandatoryBreakTest").section(&"marksRequiredAfterLineFeed");
        let o = SimpleCharacterLineBreakAnalyzer::new().analyze(&concat!("a\n",
"b"));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", o[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", o[0usize].kind.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn collapses_crlf_to_a_single_break_after_lf() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.collapsesCrlfToASingleBreakAfterLf", "org.tiqian.linebreak.MandatoryBreakTest.collapsesCrlfToASingleBreakAfterLf", || {
        TestTraceRecorder::new("MandatoryBreakTest").section(&"collapsesCrlfToASingleBreakAfterLf");
        let o = SimpleCharacterLineBreakAnalyzer::new().analyze(&concat!("a\r\n",
"b"));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Allowed", o[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", o[2usize].kind.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn preserves_each_blank_line_break() {
    testlib::run("org.tiqian.linebreak.MandatoryBreakTest.preservesEachBlankLineBreak", "org.tiqian.linebreak.MandatoryBreakTest.preservesEachBlankLineBreak", || {
        TestTraceRecorder::new("MandatoryBreakTest").section(&"preservesEachBlankLineBreak");
        let o = SimpleCharacterLineBreakAnalyzer::new().analyze(&concat!("a\n",
"\n",
"b"));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", o[1usize].kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Required", o[2usize].kind.name().to_string().as_str(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct MandatoryBreakTestHelpers;

impl MandatoryBreakTestHelpers {
    pub fn mandatory_break_test_helpers_u(cp: u32) -> String {
        let s = format!("{:0w$X}", cp, w = usize::try_from(4).unwrap_or_default());
        return format!("{}{}",
            "U+",
            s
        );
    }
}
