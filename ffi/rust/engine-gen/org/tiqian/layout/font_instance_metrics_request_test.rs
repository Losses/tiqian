#![cfg(test)]

use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::font_instance_metrics_request_test_support::FontInstanceMetricsRequestTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        FontInstanceMetricsRequestTestRubyMetricsUseTheSameItalicInstanceAsRubyShapingFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        FontInstanceMetricsRequestTestPerSpanWeightAndItalicReachTheMetricsResolverFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        FontInstanceMetricsRequestTestFaceSelectionUsesTheDisplayTextThatWasActuallyShapedFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn per_span_weight_and_italic_reach_the_metrics_resolver() {
    testlib::run("org.tiqian.layout.FontInstanceMetricsRequestTest.perSpanWeightAndItalicReachTheMetricsResolver", "org.tiqian.layout.FontInstanceMetricsRequestTest.perSpanWeightAndItalicReachTheMetricsResolver", || {
        let mut t = TestTraceRecorder::new("FontInstanceMetricsRequestTest");
        t.section(&"perSpanWeightAndItalicReachTheMetricsResolver");
        let base = FontInstanceMetricsRequestTestSupport::font_instance_metrics_request_test_support_base_style();
        FontInstanceMetricsRequestTestSupport::font_instance_metrics_request_test_support_recording_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中A", Some(vec![
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), TextStyle::new(Some(vec!["Fixture Sans".to_string()]), Some(18.0f64), Some("zh-Hans".to_string()), Some(700), Some(true), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![])), Some((base).clone()), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(180.0f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut cjk = false;
        let mut latin = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((match u32::try_from(crate::org::tiqian::layout::font_instance_metrics_request_test_support::FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner()).len()) {
Ok(value) => value, Err(_) => u32::MAX }).to_ne_bytes())) {
            let r = (crate::org::tiqian::layout::font_instance_metrics_request_test_support::FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner())[match usize::try_from(i) { Ok(value) => value, Err(_) => 0usize }]).clone();
            if r.role == FontRole::CjkText && r.font_weight == 400 && !r.italic && (r.face_selection_text).to_string() == "中" {
                cjk = true;
            }
            if r.role == FontRole::LatinText && r.font_weight == 700 && r.italic && (r.face_selection_text).to_string() == "A" {
                latin = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(cjk, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(latin, None).unwrap();
    });
}

#[test]
fn face_selection_uses_the_display_text_that_was_actually_shaped() {
    testlib::run("org.tiqian.layout.FontInstanceMetricsRequestTest.faceSelectionUsesTheDisplayTextThatWasActuallyShaped", "org.tiqian.layout.FontInstanceMetricsRequestTest.faceSelectionUsesTheDisplayTextThatWasActuallyShaped", || {
        let mut t = TestTraceRecorder::new("FontInstanceMetricsRequestTest");
        t.section(&"faceSelectionUsesTheDisplayTextThatWasActuallyShaped");
        FontInstanceMetricsRequestTestSupport::font_instance_metrics_request_test_support_recording_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("——", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])),
Some(TextStyle::new(Some(vec!["Fixture Sans".to_string()]), Some(18.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None,
Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(180.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut found = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((match u32::try_from(crate::org::tiqian::layout::font_instance_metrics_request_test_support::FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner()).len()) {
Ok(value) => value, Err(_) => u32::MAX }).to_ne_bytes())) {
            let r = (crate::org::tiqian::layout::font_instance_metrics_request_test_support::FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner())[match usize::try_from(i) { Ok(value) => value, Err(_) => 0usize }]).clone();
            if r.face_selection_text.to_string() == "⸺" {
                found = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(found, None).unwrap();
    });
}

#[test]
fn ruby_metrics_use_the_same_italic_instance_as_ruby_shaping() {
    testlib::run("org.tiqian.layout.FontInstanceMetricsRequestTest.rubyMetricsUseTheSameItalicInstanceAsRubyShaping", "org.tiqian.layout.FontInstanceMetricsRequestTest.rubyMetricsUseTheSameItalicInstanceAsRubyShaping", || {
        let mut t = TestTraceRecorder::new("FontInstanceMetricsRequestTest");
        t.section(&"rubyMetricsUseTheSameItalicInstanceAsRubyShaping");
        FontInstanceMetricsRequestTestSupport::font_instance_metrics_request_test_support_recording_engine().unwrap().layout(LayoutInput::new(TiqianTextContent::new("中", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])),
Some(TextStyle::new(Some(vec!["Fixture Sans".to_string()]), Some(18.0f64), Some("zh-Hans".to_string()), Some(400), Some(true), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None,
Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(180.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]),
Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "zhōng", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some(vec![]), Some(vec![]))).unwrap();
        let mut found = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((match u32::try_from(crate::org::tiqian::layout::font_instance_metrics_request_test_support::FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner()).len()) {
Ok(value) => value, Err(_) => u32::MAX }).to_ne_bytes())) {
            let r = (crate::org::tiqian::layout::font_instance_metrics_request_test_support::FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner())[match usize::try_from(i) { Ok(value) => value, Err(_) => 0usize }]).clone();
            if r.role == FontRole::LatinText && (r.face_selection_text).to_string() == "zhōng" && r.italic {
                found = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(found, None).unwrap();
    });
}
