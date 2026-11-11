#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::zero_width_break_control_layout_test_support::ZeroWidthBreakControlLayoutTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault) -> Self {
        match value {
            ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ZeroWidthBreakControlLayoutTestLeadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[test]
fn zero_width_space_is_unshaped_and_provides_a_soft_break_after_it() {
    testlib::run("org.tiqian.layout.ZeroWidthBreakControlLayoutTest.zeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterIt", "org.tiqian.layout.ZeroWidthBreakControlLayoutTest.zeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterIt", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[90,101,114,111,87,105,100,116,104,66,114,101,97,107,67,111,110,116,114,111,108,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[122,101,114,111,87,105,100,116,104,83,112,97,99,101,73,115,85,110,115,104,97,112,101,100,65,110,100,80,114,111,118,105,100,101,115,65,83,111,102,116,66,114,101,97,107,65,102,116,101,114,73,116]));
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = ZeroWidthBreakControlLayoutTestSupport::zero_width_break_control_layout_test_support_layout(UStr::new(&[102,111,111,8203,98,97,114]), 48.0f64, (*breaker).clone()).unwrap();
            let mut control: Option<Cluster> = None;
            for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let c = (result.clusters[usize::try_from(i).unwrap_or(0)]).clone();
                if c.text.to_ustring() == UString::from("​") {
                    control = Some(c.clone());
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), (control.as_ref().unwrap().display_text).to_ustring().as_ustr(), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.0f64, control.as_ref().unwrap().advance, 0.001f64, Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let mut shaped = false;
            for ri in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let run = (result.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
                for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if run.glyphs[usize::try_from(gi).unwrap_or(0)].clone().cluster_range.clone() == (control.as_ref().unwrap().range).clone() {
                        shaped = true;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(!shaped, Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,52,41]), UString::from(format!("{}", ((result.lines[0usize]).clone().range).clone().to_string()).as_str()).as_ustr(), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,52,44,32,101,110,100,61,55,41]), UString::from(format!("{}", ((result.lines[1usize]).clone().range).clone().to_string()).as_str()).as_ustr(), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let mut reason = UString::new();
            for i in 0..match u32::try_from((result.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((result.debug).clone().shaping_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if d.range.clone() == (control.as_ref().unwrap().range).clone() {
                    reason = (d.reason).to_ustring();
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[90,101,114,111,87,105,100,116,104,83,112,97,99,101,83,111,102,116,66,114,101,97,107,78,111,83,104,97,112,101]), reason.as_ustr(), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", (control.as_ref().unwrap().range).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (((result.debug).clone().zero_width_break_decisions[0usize]).clone().range).clone().to_string()).as_str()).as_ustr(), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
        }
    });
}

#[test]
fn leading_zero_width_space_cannot_create_an_empty_auto_wrapped_line() {
    testlib::run("org.tiqian.layout.ZeroWidthBreakControlLayoutTest.leadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLine", "org.tiqian.layout.ZeroWidthBreakControlLayoutTest.leadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[90,101,114,111,87,105,100,116,104,66,114,101,97,107,67,111,110,116,114,111,108,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[108,101,97,100,105,110,103,90,101,114,111,87,105,100,116,104,83,112,97,99,101,67,97,110,110,111,116,67,114,101,97,116,101,65,110,69,109,112,116,121,65,117,116,111,87,114,97,112,112,101,100,76,105,110,101]));
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = ZeroWidthBreakControlLayoutTestSupport::zero_width_break_control_layout_test_support_layout(UStr::new(&[8203,20013]), 8.0f64, (*breaker).clone()).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,50,41]), UString::from(format!("{}", ((result.lines[0usize]).clone().range).clone().to_string()).as_str()).as_ustr(), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
        }
    });
}
