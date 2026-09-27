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


#[derive(Debug, Clone, PartialEq)]
pub enum ZeroWidthBreakControlLayoutTestZeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterItFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
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
        let mut t = TestTraceRecorder::new("ZeroWidthBreakControlLayoutTest");
        t.section(&"zeroWidthSpaceIsUnshapedAndProvidesASoftBreakAfterIt");
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = ZeroWidthBreakControlLayoutTestSupport::zero_width_break_control_layout_test_support_layout(&"foo​bar", 48.0f64, (*breaker).clone()).unwrap();
            let mut control: Option<Cluster> = None;
            for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let c = (result.clusters[usize::try_from(i).unwrap_or(0)]).clone();
                if c.text.to_string() == "​" {
                    control = Some(c.clone());
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"", (control.as_ref().unwrap().display_text).to_string().as_str(), Some((breaker.get_strategy_name()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.0f64, control.as_ref().unwrap().advance, 0.001f64, Some((breaker.get_strategy_name()).to_string())).unwrap();
            let mut shaped = false;
            for ri in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let run = (result.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
                for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if run.glyphs[usize::try_from(gi).unwrap_or(0)].clone().cluster_range.clone() == (control.as_ref().unwrap().range).clone() {
                        shaped = true;
                    }
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(!shaped, Some((breaker.get_strategy_name()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=4)", ((result.lines[0usize]).clone().range).clone().to_string().as_str(), Some((breaker.get_strategy_name()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=4, end=7)", ((result.lines[1usize]).clone().range).clone().to_string().as_str(), Some((breaker.get_strategy_name()).to_string())).unwrap();
            let mut reason = String::new();
            for i in 0..match u32::try_from((result.debug).clone().shaping_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((result.debug).clone().shaping_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if d.range.clone() == (control.as_ref().unwrap().range).clone() {
                    reason = (d.reason).to_string();
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ZeroWidthSpaceSoftBreakNoShape", reason.as_str(), Some((breaker.get_strategy_name()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered((control.as_ref().unwrap().range).clone().to_string().as_str(), (((result.debug).clone().zero_width_break_decisions[0usize]).clone().range).clone().to_string().as_str(),
Some((breaker.get_strategy_name()).to_string())).unwrap();
        }
    });
}

#[test]
fn leading_zero_width_space_cannot_create_an_empty_auto_wrapped_line() {
    testlib::run("org.tiqian.layout.ZeroWidthBreakControlLayoutTest.leadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLine", "org.tiqian.layout.ZeroWidthBreakControlLayoutTest.leadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLine", || {
        let mut t = TestTraceRecorder::new("ZeroWidthBreakControlLayoutTest");
        t.section(&"leadingZeroWidthSpaceCannotCreateAnEmptyAutoWrappedLine");
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = ZeroWidthBreakControlLayoutTestSupport::zero_width_break_control_layout_test_support_layout(&"​中", 8.0f64, (*breaker).clone()).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((breaker.get_strategy_name()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=2)", ((result.lines[0usize]).clone().range).clone().to_string().as_str(), Some((breaker.get_strategy_name()).to_string())).unwrap();
        }
    });
}
