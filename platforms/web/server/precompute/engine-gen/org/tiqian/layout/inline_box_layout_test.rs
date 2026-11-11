#![cfg(test)]

use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::layout_queries::LayoutQueries;
use crate::org::tiqian::layout::inline_box_layout_test_support::InlineBoxLayoutTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineBoxLayoutTestInlineEdgesReserveAdvanceAndMoveTheGlyphOriginFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InlineBoxLayoutTestEveryNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCodeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,66,111,120,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[105,110,108,105,110,101,69,100,103,101,115,82,101,115,101,114,118,101,65,100,118,97,110,99,101,65,110,100,77,111,118,101,84,104,101,71,108,121,112,104,79,114,105,103,105,110]));
        let plain = InlineBoxLayoutTestSupport::inline_box_layout_test_support_plain().unwrap();
        let boxed = InlineBoxLayoutTestSupport::inline_box_layout_test_support_boxed_edges().unwrap();
        let p = (plain.clusters[1usize]).clone();
        let b = (boxed.clusters[1usize]).clone();
        let positioned = (LayoutQueries::layout_queries_positioned_clusters((boxed).clone())[1usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(p.advance + format!("{}", (8i32)).parse::<f64>().unwrap_or(0.0), b.advance, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(3 as f64, b.leading_layout_advance, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(positioned.left + format!("{}", (3i32)).parse::<f64>().unwrap_or(0.0), positioned.draw_x, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((boxed.debug).clone().inline_box_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,108,105,110,101,66,111,120,66,111,117,110,100,97,114,121,65,100,118,97,110,99,101]), (((boxed.debug).clone().inline_box_decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn every_narrow_inline_box_gets_outer_autospace_without_role_specific_code() {
    testlib::run("org.tiqian.layout.InlineBoxLayoutTest.everyNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCode", "org.tiqian.layout.InlineBoxLayoutTest.everyNarrowInlineBoxGetsOuterAutospaceWithoutRoleSpecificCode", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[73,110,108,105,110,101,66,111,120,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[101,118,101,114,121,78,97,114,114,111,119,73,110,108,105,110,101,66,111,120,71,101,116,115,79,117,116,101,114,65,117,116,111,115,112,97,99,101,87,105,116,104,111,117,116,82,111,108,101,83,112,101,99,105,102,105,99,67,111,100,101]));
        let boxed = InlineBoxLayoutTestSupport::inline_box_layout_test_support_layout(InlineBoxOuterSpacing::Narrow).unwrap();
        let mut reasons: Vec<UString> = vec![];
        let mut roles = true;
        for _g_index in 0..match u32::try_from((boxed.debug).clone().auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((boxed.debug).clone().auto_space_decisions[usize::try_from(_g_index).unwrap_or(0)]).clone();
            reasons.push((d.reason).to_ustring());
            if d.boundary_role.to_ustring() != UString::from("InlineBox.Narrow") {
                roles = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![
    UString::from("InlineBoxOuterAutoSpace:leading-W-N").to_ustring(),
    UString::from("InlineBoxOuterAutoSpace:trailing-N-W").to_ustring(),
], &reasons, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(roles, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[78,97,114,114,111,119]), (((boxed.debug).clone().inline_box_decisions[0usize]).clone().outer_spacing).to_ustring().as_ustr(), None).unwrap();
        let source = InlineBoxLayoutTestSupport::inline_box_layout_test_support_layout(InlineBoxOuterSpacing::Source).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((source.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[83,111,117,114,99,101]), (((source.debug).clone().inline_box_decisions[0usize]).clone().outer_spacing).to_ustring().as_ustr(), None).unwrap();
    });
}
