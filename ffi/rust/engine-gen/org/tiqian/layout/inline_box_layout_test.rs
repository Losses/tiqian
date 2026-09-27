#![cfg(test)]

use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::layout_queries::LayoutQueries;
use crate::org::tiqian::layout::inline_box_layout_test_support::InlineBoxLayoutTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault) -> Self {
        match value {
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault) -> Self {
        match value {
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault) -> Self {
        match value {
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault) -> Self {
        match value {
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault) -> Self {
        match value {
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault) -> Self {
        match value {
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault) -> Self {
        match value {
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault) -> Self {
        match value {
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn inline_edges_reserve_advance_and_move_the_glyph_origin() {
    testlib::run("org.tiqian.layout.InlineBoxLayoutTest.inlineEdgesReserveAdvanceAndMoveTheGlyphOrigin", "org.tiqian.layout.InlineBoxLayoutTest.inlineEdgesReserveAdvanceAndMoveTheGlyphOrigin", || {
        let mut t = TestTraceRecorder::new("InlineBoxLayoutTest");
        t.section(&"inlineEdgesReserveAdvanceAndMoveTheGlyphOrigin");
        let plain = InlineBoxLayoutTestSupport::inline_box_layout_test_support_plain().unwrap();
        let boxed = InlineBoxLayoutTestSupport::inline_box_layout_test_support_boxed_edges().unwrap();
        let p = (plain.clusters[1usize]).clone();
        let b = (boxed.clusters[1usize]).clone();
        let positioned = (LayoutQueries::layout_queries_positioned_clusters((boxed).clone())[1usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(p.advance + format!("{}", (8i32)).parse::<f64>().unwrap_or(0.0), b.advance, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(3 as f64, b.leading_layout_advance, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(positioned.left + format!("{}", (3i32)).parse::<f64>().unwrap_or(0.0), positioned.draw_x, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((boxed.debug).clone().inline_box_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InlineBoxBoundaryAdvance", (((boxed.debug).clone().inline_box_decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn every_narrow_inline_box_gets_outer_autospace_without_role_specific_code() {
    testlib::run("org.tiqian.layout.InlineBoxLayoutTest.everyNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCode", "org.tiqian.layout.InlineBoxLayoutTest.everyNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCode", || {
        let mut t = TestTraceRecorder::new("InlineBoxLayoutTest");
        t.section(&"everyNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCode");
        let boxed = InlineBoxLayoutTestSupport::inline_box_layout_test_support_layout(InlineBoxOuterSpacing::Narrow).unwrap();
        let mut reasons: Vec<String> = vec![];
        let mut roles = true;
        for _g_index in 0..match u32::try_from((boxed.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((boxed.debug).clone().auto_space_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            reasons.push((d.reason).to_string());
            if d.boundary_role.to_string() != "InlineBox.Narrow" {
                roles = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![
    "InlineBoxOuterAutoSpace:leading-W-N".to_string(),
    "InlineBoxOuterAutoSpace:trailing-N-W".to_string(),
], &reasons, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(roles, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Narrow", (((boxed.debug).clone().inline_box_decisions[0usize]).clone().outer_spacing).to_string().as_str(), None).unwrap();
        let source = InlineBoxLayoutTestSupport::inline_box_layout_test_support_layout(InlineBoxOuterSpacing::Source).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((source.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Source", (((source.debug).clone().inline_box_decisions[0usize]).clone().outer_spacing).to_string().as_str(), None).unwrap();
    });
}
