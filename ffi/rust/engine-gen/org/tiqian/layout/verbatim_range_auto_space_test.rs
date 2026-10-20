#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault> for crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault {
    fn from(value: VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault> for VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    fn from(value: crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault) -> Self {
        VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault> for crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault {
    fn from(value: VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault) -> Self {
        match value {
            VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault> for VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault {
    fn from(value: crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault) -> Self {
        VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        VerbatimRangeAutoSpaceTestInternalBoundariesAreSuppressedAndOuterEdgesKeepTheGapFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[test]
fn internal_boundaries_are_suppressed_and_outer_edges_keep_the_gap() {
    testlib::run("org.tiqian.layout.VerbatimRangeAutoSpaceTest.internalBoundariesAreSuppressedAndOuterEdgesKeepTheGap", "org.tiqian.layout.VerbatimRangeAutoSpaceTest.internalBoundariesAreSuppressedAndOuterEdgesKeepTheGap", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[86,101,114,98,97,116,105,109,82,97,110,103,101,65,117,116,111,83,112,97,99,101,84,101,115,116])));
        t.section(UStr::new(&[105,110,116,101,114,110,97,108,66,111,117,110,100,97,114,105,101,115,65,114,101,83,117,112,112,114,101,115,115,101,100,65,110,100,79,117,116,101,114,69,100,103,101,115,75,101,101,112,84,104,101,71,97,112]));
        let text = UString::from("跑print你好print跑").to_ustring();
        let control = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_ustr(), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((control).clone(), UStr::new(&[84,101,120,116,65,117,116,111,83,112,97,99,101,73,110,115,101,114,116,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,78])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", control.to_string()).as_str()).as_ustr(); __s += &(UString::from(".debug.autoSpaceDecisions")); __s }).as_str()))).unwrap();
        let result = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_ustr(), &vec![(TextRange::new(1u32, 13u32).unwrap()).clone()]).unwrap();
        let decisions = ((result.debug).clone().auto_space_decisions).clone();
        let mut decision_parts: Vec<UString> = vec![];
        let mut di = 0u32;
        while (i32::from_ne_bytes(((di) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            decision_parts.push(UString::from(format!("{}", (decisions[usize::try_from(di).unwrap_or(0)]).clone().to_string()).as_str()));
            di = u32::wrapping_add(di, 1);
        }
        let decisions_text = TestTraceRender::test_trace_render_legacy_list_text(&decision_parts);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((result).clone(), UStr::new(&[84,101,120,116,65,117,116,111,83,112,97,99,101,73,110,115,101,114,116,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,78])), Some((decisions_text).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((result).clone(), UStr::new(&[86,101,114,98,97,116,105,109,82,97,110,103,101,65,117,116,111,83,112,97,99,101,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,78,45,115,117,112,112,114,101,115,115,101,100])), Some((decisions_text).to_ustring())).unwrap();
    });
}

#[test]
fn typed_space_inside_a_verbatim_range_is_not_normalised() {
    testlib::run("org.tiqian.layout.VerbatimRangeAutoSpaceTest.typedSpaceInsideAVerbatimRangeIsNotNormalised", "org.tiqian.layout.VerbatimRangeAutoSpaceTest.typedSpaceInsideAVerbatimRangeIsNotNormalised", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[86,101,114,98,97,116,105,109,82,97,110,103,101,65,117,116,111,83,112,97,99,101,84,101,115,116])));
        t.section(UStr::new(&[116,121,112,101,100,83,112,97,99,101,73,110,115,105,100,101,65,86,101,114,98,97,116,105,109,82,97,110,103,101,73,115,78,111,116,78,111,114,109,97,108,105,115,101,100]));
        let text = UString::from("跑a 你b跑").to_ustring();
        let control = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_ustr(), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((control).clone(), UStr::new(&[84,101,120,116,65,117,116,111,83,112,97,99,101,82,101,112,108,97,99,101,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,115,112,97,99,101,45,78])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", control.to_string()).as_str()).as_ustr(); __s += &(UString::from(".debug.autoSpaceDecisions")); __s }).as_str()))).unwrap();
        let result = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_ustr(), &vec![(TextRange::new(1u32, 5u32).unwrap()).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((result).clone(), UStr::new(&[84,101,120,116,65,117,116,111,83,112,97,99,101,82,101,112,108,97,99,101,58,101,97,115,116,45,97,115,105,97,110,45,115,112,97,99,105,110,103,45,87,45,115,112,97,99,101,45,78])), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", result.to_string()).as_str()).as_ustr(); __s += &(UString::from(".debug.autoSpaceDecisions")); __s }).as_str()))).unwrap();
    });
}
