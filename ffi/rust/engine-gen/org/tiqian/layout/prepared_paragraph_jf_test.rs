#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::decoration_decision_info::DecorationDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_debug_info::LayoutDebugInfo;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::size::Size;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphFns;
use crate::org::tiqian::layout::prepared_paragraph_jf_test_support::PreparedParagraphJfTestSupport;
use crate::org::tiqian::protocol::plan_json_number::PlanJsonNumber;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphJfTestStyleAtAndStyleDeltasInPreparedParagraphJsonFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault) -> Self {
        match value {
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault) -> Self {
        match value {
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault) -> Self {
        match value {
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault) -> Self {
        match value {
            PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphJfTestInlineBoxEdgesAndEmphasisDotsFilterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault) -> Self {
        match value {
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault) -> Self {
        match value {
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault) -> Self {
        match value {
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault) -> Self {
        match value {
            PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphJfTestDashShapingDecisionWithGlyphIdsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn dash_shaping_decision_with_glyph_ids() {
    testlib::run("org.tiqian.layout.PreparedParagraphJfTest.dashShapingDecisionWithGlyphIds", "org.tiqian.layout.PreparedParagraphJfTest.dashShapingDecisionWithGlyphIds", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,74,102,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,83,104,97,112,105,110,103,68,101,99,105,115,105,111,110,87,105,116,104,71,108,121,112,104,73,100,115]));
        let c = TiqianTextContent::new(&(UStr::new(&[8212,8212])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let r = LayoutResult::new(LayoutInput::new((c).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Size::new(200 as f64 as f64, 24 as f64 as f64), vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[8212,8212])), &(UStr::new(&[107])), 32 as f64 as f64, Some(UString::from("——")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[107])), vec![
    (Glyph::new(42u32, TextRange::new(0u32, 2u32).unwrap(), 32 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
], vec![
    (PreparedParagraphJfTestSupport::prepared_paragraph_jf_test_support_line(TextRange::new(0u32, 2u32).unwrap(), IntRange::new(0u32, 0u32), Some(32 as f64 as f64))).clone(),
], LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (ShapingDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[8212,8212])), &(UStr::new(&[8212,8212])), &(UStr::new(&[107])), 1u32, 32 as f64 as f64, &(UStr::new(&[116,101,115,116])), &(UStr::new(&[68,97,115,104,82,117,108,101])), Some(0), Some(0), Some(UString::from("NotoSansCJK")), None, Some(UString::from("zh")), Some(UString::from("DashTwoEmLigature")), None, None)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"glyphIds\":\"42\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"shapingLanguage\":\"zh\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"resolvedFace\":\"NotoSansCJK\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn ecma_json_number_edge_cases() {
    testlib::run("org.tiqian.layout.PreparedParagraphJfTest.ecmaJsonNumberEdgeCases", "org.tiqian.layout.PreparedParagraphJfTest.ecmaJsonNumberEdgeCases", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,74,102,84,101,115,116])));
        t.section(UStr::new(&[101,99,109,97,74,115,111,110,78,117,109,98,101,114,69,100,103,101,67,97,115,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(TestHelpers::test_helpers_f32_bits(1)).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(TestHelpers::test_helpers_f32_bits(8388607)).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[56,46,57,57,57,57,57,57,54,56,56,53,52,48,51,48,57,101,45,49,55]), PlanJsonNumber::plan_json_number_ecma_json_number(TestHelpers::test_helpers_f32_literal(9.000000000000001e-17f64)).unwrap().as_ustr(), None).unwrap();
        for i in 1..2001 {
            PlanJsonNumber::plan_json_number_ecma_json_number(TestHelpers::test_helpers_f32_literal(format!("{}", { let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0) * 1e-17f64)).unwrap();
            PlanJsonNumber::plan_json_number_ecma_json_number(TestHelpers::test_helpers_f32_literal(format!("{}", { let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0) * 1e-15f64)).unwrap();
            PlanJsonNumber::plan_json_number_ecma_json_number(TestHelpers::test_helpers_f32_literal(format!("{}", { let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0) * 1e-20f64)).unwrap();
        }
        for shift in 1..61 {
            let v = TestHelpers::test_helpers_f32_literal(f64::powf(2.0f64, { let v: u32 = shift; i32::from_ne_bytes(v.to_ne_bytes()) } as f64));
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(v).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(-v).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
            let inv = TestHelpers::test_helpers_f32_literal(1.0f64 / v);
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(inv).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(-inv).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        }
        let edge_values = vec![
    0.9999999999999999f64,
    0.09999999999999999f64,
    0.009999999999999999f64,
    9.999999999999999e-5f64,
    1.9999999999999998e-4f64,
    9.999999999999999e20f64,
    1.9999999999999999e20f64,
    1e-300f64,
    1e300f64,
    1.401298464324817e-45f64,
    3.4028234663852886e38f64,
    1.0e-302f64,
    5.960464477539063e-8f64,
    2.9802322387695312e-8f64,
];
        for &raw in &edge_values {
            let v = TestHelpers::test_helpers_f32_literal(raw);
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(v).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(PlanJsonNumber::plan_json_number_ecma_json_number(-v).unwrap()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        }
    });
}

#[test]
fn inline_box_edges_and_emphasis_dots_filter() {
    testlib::run("org.tiqian.layout.PreparedParagraphJfTest.inlineBoxEdgesAndEmphasisDotsFilter", "org.tiqian.layout.PreparedParagraphJfTest.inlineBoxEdgesAndEmphasisDotsFilter", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,74,102,84,101,115,116])));
        t.section(UStr::new(&[105,110,108,105,110,101,66,111,120,69,100,103,101,115,65,110,100,69,109,112,104,97,115,105,115,68,111,116,115,70,105,108,116,101,114]));
        let c = TiqianTextContent::new(&(UStr::new(&[30002,20057])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let cs = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[30002])), &(UStr::new(&[107])), 16 as f64 as f64, Some(UString::from("甲")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), &(UStr::new(&[20057])), &(UStr::new(&[107])), 16 as f64 as f64, Some(UString::from("乙")), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let debug = LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (DecorationDecisionInfo::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[30002])), &(UStr::new(&[69,109,112,104,97,115,105,115])), false, &(UStr::new(&[116,101,115,116])), Some(0.0), Some(0.0), Some(4 as f64))).clone(),
    (DecorationDecisionInfo::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[30002])), &(UStr::new(&[80,114,111,112,101,114,78,111,117,110])), true, &(UStr::new(&[116,101,115,116])), Some(0.0), Some(0.0), Some(4 as f64))).clone(),
    (DecorationDecisionInfo::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[30002])), &(UStr::new(&[69,109,112,104,97,115,105,115])), true, &(UStr::new(&[116,101,115,116])), Some(0.0), Some(0.0), Some(0 as f64))).clone(),
    (DecorationDecisionInfo::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[30002])), &(UStr::new(&[69,109,112,104,97,115,105,115])), true, &(UStr::new(&[116,101,115,116])), Some(8 as f64), Some(20 as f64), Some(4 as f64))).clone(),
]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let r = LayoutResult::new(LayoutInput::new((c).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(4 as f64), Some(0 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(1u32, 2u32).unwrap(), Some(0 as f64), Some(6 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some(vec![])), Size::new(200 as f64 as f64, 24 as f64 as f64), (cs).clone(), vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[107])), vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32).unwrap(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(1u32, 2u32).unwrap(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
], vec![
    (PreparedParagraphJfTestSupport::prepared_paragraph_jf_test_support_line(TextRange::new(0u32, 2u32).unwrap(), IntRange::new(0u32, 1u32), Some(32 as f64 as f64))).clone(),
], (debug).clone());
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"inlineStart\":4").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"inlineEnd\":6").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"emphasisDots\":").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn style_at_and_style_deltas_in_prepared_paragraph_json() {
    testlib::run("org.tiqian.layout.PreparedParagraphJfTest.styleAtAndStyleDeltasInPreparedParagraphJson", "org.tiqian.layout.PreparedParagraphJfTest.styleAtAndStyleDeltasInPreparedParagraphJson", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,74,102,84,101,115,116])));
        t.section(UStr::new(&[115,116,121,108,101,65,116,65,110,100,83,116,121,108,101,68,101,108,116,97,115,73,110,80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,74,115,111,110]));
        let content = TiqianTextContent::new(&(UStr::new(&[30002,20057,19993,19969,25098,24049])), Some(vec![
    (TextSpan::new(TextRange::new(1u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(20 as f64), Some(UString::from("zh-Hans")), Some(700), Some(true), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new(TextRange::new(2u32, 4u32).unwrap(), TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new(TextRange::new(4u32, 5u32).unwrap(), TextStyle::new(Some(vec![]), Some(16 as f64), Some(UString::from("zh-Hans")), Some(400), Some(true), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        let texts_5 = UString::from("己").to_ustring();
        let texts_4 = UString::from("戊").to_ustring();
        let texts_3 = UString::from("丁").to_ustring();
        let texts_2 = UString::from("丙").to_ustring();
        let texts_1 = UString::from("乙").to_ustring();
        let texts_0 = UString::from("甲").to_ustring();
        let mut _g: Vec<Cluster> = vec![];
        {
            _g.push(Cluster::new(TextRange::new(0u32, 1u32).unwrap(), texts_0.as_ustr(), &(UStr::new(&[107])), 16 as f64 as f64, Some(texts_0.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
            _g.push(Cluster::new(TextRange::new(1u32, 2u32).unwrap(), texts_1.as_ustr(), &(UStr::new(&[107])), 20 as f64 as f64, Some(texts_1.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
            _g.push(Cluster::new(TextRange::new(2u32, 3u32).unwrap(), texts_2.as_ustr(), &(UStr::new(&[107])), 16 as f64 as f64, Some(texts_2.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
            _g.push(Cluster::new(TextRange::new(3u32, 4u32).unwrap(), texts_3.as_ustr(), &(UStr::new(&[107])), 16 as f64 as f64, Some(texts_3.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
            _g.push(Cluster::new(TextRange::new(4u32, 5u32).unwrap(), texts_4.as_ustr(), &(UStr::new(&[107])), 16 as f64 as f64, Some(texts_4.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
            _g.push(Cluster::new(TextRange::new(5u32, 6u32).unwrap(), texts_5.as_ustr(), &(UStr::new(&[107])), 16 as f64 as f64, Some(texts_5.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
        }
        let clusters = (_g).clone();
        let mut _g: Vec<Glyph> = vec![];
        {
            _g.push(Glyph::new(1u32, TextRange::new(0u32, 1u32).unwrap(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None));
            _g.push(Glyph::new(2u32, TextRange::new(1u32, 2u32).unwrap(), 20 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None));
            _g.push(Glyph::new(3u32, TextRange::new(2u32, 3u32).unwrap(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None));
            _g.push(Glyph::new(4u32, TextRange::new(3u32, 4u32).unwrap(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None));
            _g.push(Glyph::new(5u32, TextRange::new(4u32, 5u32).unwrap(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None));
            _g.push(Glyph::new(6u32, TextRange::new(5u32, 6u32).unwrap(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None));
        }
        let glyphs = (_g).clone();
        let r = LayoutResult::new(LayoutInput::new((content).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Size::new(200 as f64 as f64, 24 as f64 as f64), (clusters).clone(), vec![
    (GlyphRun::new(TextRange::new(0u32, 6u32).unwrap(), &(UStr::new(&[107])), glyphs.to_vec(), 100 as f64 as f64, Some(vec![UString::from("liga").to_ustring(), UString::from("dlig").to_ustring()]))).clone(),
], vec![
    (PreparedParagraphJfTestSupport::prepared_paragraph_jf_test_support_line(TextRange::new(0u32, 6u32).unwrap(), IntRange::new(0u32, 5u32), Some(100 as f64 as f64))).clone(),
],
LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"openTypeFeatures\":[\"liga\",\"dlig\"]").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"fontSize\":20").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"fontWeight\":700").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(json), UString::from("\"italic\":true").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}
