#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum VerbatimRangeAutoSpaceTestTypedSpaceInsideAVerbatimRangeIsNotNormalisedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::verbatim_range_auto_space_test_support::VerbatimRangeAutoSpaceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
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
        let mut t = TestTraceRecorder::new("VerbatimRangeAutoSpaceTest");
        t.section(&"internalBoundariesAreSuppressedAndOuterEdgesKeepTheGap");
        let text = "跑print你好print跑".to_string();
        let control = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_str(), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((control).clone(), &"TextAutoSpaceInsert:east-asian-spacing-W-N"), Some((format!("{}{}",
            control.to_string(),
            ".debug.autoSpaceDecisions"
        )).to_string())).unwrap();
        let result = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_str(), &vec![(TextRange::new(1u32, 13u32).unwrap()).clone()]).unwrap();
        let decisions = ((result.debug).clone().auto_space_decisions).clone();
        let mut decision_parts: Vec<String> = vec![];
        let mut di = 0u32;
        while (i32::from_ne_bytes((di).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            decision_parts.push((decisions[usize::try_from(di).unwrap_or(0)]).clone().to_string());
            di = u32::wrapping_add(di, 1);
        }
        let decisions_text = TestTraceRender::test_trace_render_legacy_list_text(&decision_parts);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((result).clone(), &"TextAutoSpaceInsert:east-asian-spacing-W-N"), Some((decisions_text).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((result).clone(), &"VerbatimRangeAutoSpace:east-asian-spacing-W-N-suppressed"), Some((decisions_text).to_string())).unwrap();
    });
}

#[test]
fn typed_space_inside_a_verbatim_range_is_not_normalised() {
    testlib::run("org.tiqian.layout.VerbatimRangeAutoSpaceTest.typedSpaceInsideAVerbatimRangeIsNotNormalised", "org.tiqian.layout.VerbatimRangeAutoSpaceTest.typedSpaceInsideAVerbatimRangeIsNotNormalised", || {
        let mut t = TestTraceRecorder::new("VerbatimRangeAutoSpaceTest");
        t.section(&"typedSpaceInsideAVerbatimRangeIsNotNormalised");
        let text = "跑a 你b跑".to_string();
        let control = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_str(), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((control).clone(), &"TextAutoSpaceReplace:east-asian-spacing-W-space-N"), Some((format!("{}{}",
            control.to_string(),
            ".debug.autoSpaceDecisions"
        )).to_string())).unwrap();
        let result = VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_layout(text.as_str(), &vec![(TextRange::new(1u32, 5u32).unwrap()).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, VerbatimRangeAutoSpaceTestSupport::verbatim_range_auto_space_test_support_count((result).clone(), &"TextAutoSpaceReplace:east-asian-spacing-W-space-N"), Some((format!("{}{}",
            result.to_string(),
            ".debug.autoSpaceDecisions"
        )).to_string())).unwrap();
    });
}
