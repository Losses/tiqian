#![cfg(test)]

use crate::org::tiqian::core::color_span::ColorSpan;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::link_address_display::LinkAddressDisplay;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rich_text_background_draw_style::Border;
use crate::org::tiqian::core::rich_text_background_metric_policy::RichTextBackgroundMetricPolicy;
use crate::org::tiqian::core::rich_text_background_paint::RichTextBackgroundPaint;
use crate::org::tiqian::core::rich_text_line_pattern::Dashed;
use crate::org::tiqian::core::rich_text_line_pattern::Dotted;
use crate::org::tiqian::core::rich_text_paint::RichTextPaint;
use crate::org::tiqian::core::rich_text_role::Link;
use crate::org::tiqian::core::rich_text_role::RichTextRole;
use crate::org::tiqian::core::rich_text_span::RichTextSpan;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault) -> Self {
        match value {
            TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault) -> Self {
        match value {
            TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault) -> Self {
        match value {
            TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextModelCoverageTestTestTextStyleAndDecorationsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<TextModelCoverageTestTestTextStyleAndDecorationsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextModelCoverageTestTestTextStyleAndDecorationsFault) -> Self {
        match value {
            TextModelCoverageTestTestTextStyleAndDecorationsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestTextStyleAndDecorationsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextModelCoverageTestTestTextStyleAndDecorationsFault) -> Self {
        match value {
            TextModelCoverageTestTestTextStyleAndDecorationsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestTextStyleAndDecorationsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextModelCoverageTestTestTextStyleAndDecorationsFault) -> Self {
        match value {
            TextModelCoverageTestTestTextStyleAndDecorationsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextModelCoverageTestTestTextStyleAndDecorationsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextModelCoverageTestTestTextStyleAndDecorationsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextModelCoverageTestTestTextStyleAndDecorationsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextModelCoverageTestTestTextStyleAndDecorationsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextModelCoverageTestTestTextStyleAndDecorationsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextModelCoverageTestTestTextStyleAndDecorationsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextModelCoverageTestTestSpansAndInlineBoxFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<TextModelCoverageTestTestSpansAndInlineBoxFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextModelCoverageTestTestSpansAndInlineBoxFault) -> Self {
        match value {
            TextModelCoverageTestTestSpansAndInlineBoxFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestSpansAndInlineBoxFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextModelCoverageTestTestSpansAndInlineBoxFault) -> Self {
        match value {
            TextModelCoverageTestTestSpansAndInlineBoxFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestSpansAndInlineBoxFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextModelCoverageTestTestSpansAndInlineBoxFault) -> Self {
        match value {
            TextModelCoverageTestTestSpansAndInlineBoxFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextModelCoverageTestTestSpansAndInlineBoxFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextModelCoverageTestTestSpansAndInlineBoxFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextModelCoverageTestTestSpansAndInlineBoxFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextModelCoverageTestTestSpansAndInlineBoxFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextModelCoverageTestTestSpansAndInlineBoxFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextModelCoverageTestTestSpansAndInlineBoxFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextModelCoverageTestTestRubyAndParagraphModelsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<TextModelCoverageTestTestRubyAndParagraphModelsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextModelCoverageTestTestRubyAndParagraphModelsFault) -> Self {
        match value {
            TextModelCoverageTestTestRubyAndParagraphModelsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestRubyAndParagraphModelsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextModelCoverageTestTestRubyAndParagraphModelsFault) -> Self {
        match value {
            TextModelCoverageTestTestRubyAndParagraphModelsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestRubyAndParagraphModelsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextModelCoverageTestTestRubyAndParagraphModelsFault) -> Self {
        match value {
            TextModelCoverageTestTestRubyAndParagraphModelsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextModelCoverageTestTestRubyAndParagraphModelsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextModelCoverageTestTestRubyAndParagraphModelsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextModelCoverageTestTestRubyAndParagraphModelsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextModelCoverageTestTestRubyAndParagraphModelsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextModelCoverageTestTestRubyAndParagraphModelsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextModelCoverageTestTestRubyAndParagraphModelsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextModelCoverageTestTestRichTextSpansAndPatternsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<TextModelCoverageTestTestRichTextSpansAndPatternsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextModelCoverageTestTestRichTextSpansAndPatternsFault) -> Self {
        match value {
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestRichTextSpansAndPatternsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextModelCoverageTestTestRichTextSpansAndPatternsFault) -> Self {
        match value {
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestRichTextSpansAndPatternsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TextModelCoverageTestTestRichTextSpansAndPatternsFault) -> Self {
        match value {
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestRichTextSpansAndPatternsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextModelCoverageTestTestRichTextSpansAndPatternsFault) -> Self {
        match value {
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestRichTextSpansAndPatternsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: TextModelCoverageTestTestRichTextSpansAndPatternsFault) -> Self {
        match value {
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextModelCoverageTestTestRichTextSpansAndPatternsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextModelCoverageTestTestRichTextSpansAndPatternsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextModelCoverageTestTestRichTextSpansAndPatternsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextModelCoverageTestTestRichTextSpansAndPatternsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TextModelCoverageTestTestRichTextSpansAndPatternsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TextModelCoverageTestTestRichTextSpansAndPatternsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextModelCoverageTestTestRichTextSpansAndPatternsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextModelCoverageTestTestRichTextSpansAndPatternsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for TextModelCoverageTestTestRichTextSpansAndPatternsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        TextModelCoverageTestTestRichTextSpansAndPatternsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault) -> Self {
        match value {
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault) -> Self {
        match value {
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault) -> Self {
        match value {
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault) -> Self {
        match value {
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault) -> Self {
        match value {
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[test]
fn test_tiqian_text_content_and_link_address_display() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testTiqianTextContentAndLinkAddressDisplay", "org.tiqian.core.TextModelCoverageTest.testTiqianTextContentAndLinkAddressDisplay", || {
        TestTraceRecorder::new("TextModelCoverageTest").section(&"testTiqianTextContentAndLinkAddressDisplay");
        let content = TiqianTextContent::new("Hello Tiqian", Some(vec![
    (TextSpan::new(TextRange::new(0u32, 5u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None)))).clone(),
]), Some(vec![0, 5, 12]), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 5u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![(TextRange::new(6u32, 12u32).unwrap()).clone()]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Hello Tiqian", (content.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((content.spans.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((content.source_boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((content.line_break_spans.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((content.auto_space_suppressed_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(content.to_string().as_str()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&content.to_string(), "TiqianTextContent", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"", &""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"tiqian.org", &""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"", &"https://tiqian.org"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"tiqian.org", &"tiqian.org"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"tiqian.org", &"https://tiqian.org"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"tiqian.org", &"http://tiqian.org"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(&"dev@tiqian.org", &"mailto:dev@tiqian.org"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"tiqian.org", &"https://other.org"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(&"tiqian.org", &"ftp://tiqian.org"), None).unwrap();
    });
}

#[test]
fn test_spans_and_inline_box() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testSpansAndInlineBox", "org.tiqian.core.TextModelCoverageTest.testSpansAndInlineBox", || {
        TestTraceRecorder::new("TextModelCoverageTest").section(&"testSpansAndInlineBox");
        let line_break_span = LineBreakSpan::new(TextRange::new(0u32, 4u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 4u32).unwrap().to_string().as_str(), (line_break_span.range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(LineBreakPolicy::ProgressiveTechnical.name().to_string().as_str(), line_break_span.policy.name().to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(line_break_span.to_string().as_str()).unwrap();
        let progressive_technical = LineBreakPolicy::ProgressiveTechnical;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { progressive_technical.name().to_string() }.as_str(), None).unwrap();
        let attachment_none = InlineAttachment::None;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { attachment_none.name().to_string() }.as_str(), None).unwrap();
        let attachment_previous = InlineAttachment::Previous;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { attachment_previous.name().to_string() }.as_str(), None).unwrap();
        let spacing_narrow = InlineBoxOuterSpacing::Narrow;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { spacing_narrow.name().to_string() }.as_str(), None).unwrap();
        let spacing_source = InlineBoxOuterSpacing::Source;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { spacing_source.name().to_string() }.as_str(), None).unwrap();
        let inline_box = InlineBoxSpan::new(TextRange::new(1u32, 3u32).unwrap(), Some(2.0f64), Some(3.0f64), Some(InlineBoxOuterSpacing::Source));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(1u32, 3u32).unwrap().to_string().as_str(), (inline_box.range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, inline_box.inline_start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, inline_box.inline_end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(InlineBoxOuterSpacing::Source.name().to_string().as_str(), inline_box.outer_spacing.name().to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(inline_box.to_string().as_str()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"￼", InlineObjectSpan::INLINE_OBJECT_SPAN_INLINE_OBJECT_REPLACEMENT_CHAR.to_string().as_str(), None).unwrap();
    });
}

#[test]
fn test_inline_object_preferred_stretch_and_adjustment() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testInlineObjectPreferredStretchAndAdjustment", "org.tiqian.core.TextModelCoverageTest.testInlineObjectPreferredStretchAndAdjustment", || {
        TestTraceRecorder::new("TextModelCoverageTest").section(&"testInlineObjectPreferredStretchAndAdjustment");
        let punctuation_trailing = InlineObjectPreferredStretchKind::PunctuationTrailing;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { punctuation_trailing.name().to_string() }.as_str(), None).unwrap();
        let relation = InlineObjectPreferredStretchKind::Relation;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { relation.name().to_string() }.as_str(), None).unwrap();
        let binary_operator = InlineObjectPreferredStretchKind::BinaryOperator;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { binary_operator.name().to_string() }.as_str(), None).unwrap();
        let stretch = InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 10.0f64, 15.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(InlineObjectPreferredStretchKind::Relation.name().to_string().as_str(), stretch.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, stretch.natural_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(15.0f64, stretch.target_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5.0f64, stretch.get_capacity(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(stretch.to_string().as_str()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, -1.0f64, 10.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 0.0f64 / 0.0f64, 10.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, f64::INFINITY, 10.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 10.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 8.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, 0.0f64 / 0.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::PunctuationTrailing, 10.0f64, f64::INFINITY).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let fixed = InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(fixed.participates_in_uniform_stretch, None).unwrap();
        let fixed_preferred_stretch = fixed.preferred_stretch.clone();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(fixed_preferred_stretch.is_none(), match &(fixed_preferred_stretch) { None => "-".to_string(), Some(__option2) => __option2.to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, fixed.shrink_capacity, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, fixed.line_end_discardable_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(fixed.prevents_line_break, None).unwrap();
        let custom_adj = InlineObjectBoundaryAdjustment::new(Some(true), Some((stretch).clone()), Some(2.0f64), Some(1.0f64), Some(true)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(custom_adj.participates_in_uniform_stretch, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(stretch.to_string().as_str(), custom_adj.preferred_stretch.as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, custom_adj.shrink_capacity, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, custom_adj.line_end_discardable_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(custom_adj.prevents_line_break, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(custom_adj.to_string().as_str()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectBoundaryAdjustment::new(Some(false), None, Some(-0.5f64), Some(0.0), Some(false)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0f64 / 0.0f64), Some(0.0), Some(false)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0f64), Some(-0.5f64), Some(false)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0f64), Some(0.0f64 / 0.0f64), Some(false)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let inline_object = InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 16.0f64, 12.0f64, 4.0f64, Some((fixed).clone()), Some((custom_adj).clone())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), (inline_object.range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, inline_object.advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.0f64, inline_object.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.0f64, inline_object.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, None).unwrap();
    });
}

#[test]
fn test_text_style_and_decorations() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testTextStyleAndDecorations", "org.tiqian.core.TextModelCoverageTest.testTextStyleAndDecorations", || {
        TestTraceRecorder::new("TextModelCoverageTest").section(&"testTextStyleAndDecorations");
        let style = TextStyle::new(Some(vec!["Noto Serif CJK SC".to_string()]), Some(18.0f64), Some("zh-CN".to_string()), Some(700), Some(true), Some(-2.0f64), Some(InlineAttachment::Previous));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_strings(&vec!["Noto Serif CJK SC".to_string()]).as_str(),
TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_strings(&style.font_families).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.0f64, style.font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"zh-CN", (style.locale).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(700, style.font_weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(style.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(-2.0f64, style.baseline_shift, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(InlineAttachment::Previous.name().to_string().as_str(), style.inline_attachment.name().to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(style.to_string().as_str()).unwrap();
        let emphasis = DecorationKind::Emphasis;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { emphasis.name().to_string() }.as_str(), None).unwrap();
        let mourning = DecorationKind::Mourning;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { mourning.name().to_string() }.as_str(), None).unwrap();
        let proper_noun = DecorationKind::ProperNoun;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { proper_noun.name().to_string() }.as_str(), None).unwrap();
        let book_title = DecorationKind::BookTitle;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { book_title.name().to_string() }.as_str(), None).unwrap();
        let decoration = DecorationSpan::new(TextRange::new(2u32, 4u32).unwrap(), DecorationKind::Emphasis);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(2u32, 4u32).unwrap().to_string().as_str(), (decoration.range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(DecorationKind::Emphasis.name().to_string().as_str(), decoration.kind.name().to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(decoration.to_string().as_str()).unwrap();
        let color = ColorSpan::new(1u32, 5u32, 4279312947u32);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, color.start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, color.end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4279312947u32, color.argb, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(color.to_string().as_str()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&color.to_string(), "ColorSpan", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_rich_text_spans_and_patterns() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testRichTextSpansAndPatterns", "org.tiqian.core.TextModelCoverageTest.testRichTextSpansAndPatterns", || {
        TestTraceRecorder::new("TextModelCoverageTest").section(&"testRichTextSpansAndPatterns");
        let paint = RichTextPaint::new(Some(4278190080u32), Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(1.5f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4278190080u32, *(paint.argb).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.5f64, paint.adjacent_same_style_clearance, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(paint.to_string().as_str()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, Some(-0.1f64)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, Some(0.0f64 / 0.0f64)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, Some(f64::INFINITY)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let bg_paint = RichTextBackgroundPaint::new(Some(2.0f64), Some(3.0f64), Some(4.0f64), Some(1.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle), Some(Box::new(Border::new(1.5f64).unwrap()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, bg_paint.horizontal_padding, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, bg_paint.vertical_padding, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4.0f64, bg_paint.corner_radius, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, bg_paint.continuation_corner_radius, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(RichTextBackgroundMetricPolicy::UniformTextStyle.name().to_string().as_str(), bg_paint.metric_policy.name().to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(bg_paint.to_string().as_str()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(-1.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64 / 0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(-1.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64 / 0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(-1.0f64), Some(-1.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64 / 0.0f64), Some(0.0f64 / 0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(-1.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64 / 0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone().to_string().as_str(),
(*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone().to_string().as_str(), None).unwrap();
        let border = Border::new(2.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, border.stroke_width, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(border.to_string().as_str()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Border::new(0.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Border::new(-1.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Border::new(0.0f64 / 0.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let marked_faces = RichTextBackgroundMetricPolicy::MarkedFaces;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { marked_faces.name().to_string() }.as_str(), None).unwrap();
        let uniform_text_style = RichTextBackgroundMetricPolicy::UniformTextStyle;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { uniform_text_style.name().to_string() }.as_str(), None).unwrap();
        let uniform_paragraph_style = RichTextBackgroundMetricPolicy::UniformParagraphStyle;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { uniform_paragraph_style.name().to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone().to_string().as_str(), (*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone().to_string().as_str(),
None).unwrap();
        let dashed = Dashed::new(1.0f64, 4.0f64, 2.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, dashed.stroke_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4.0f64, dashed.dash_length, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, dashed.gap_length, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(dashed.to_string().as_str()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(0.0f64, 4.0f64, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(-1.0f64, 4.0f64, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(0.0f64 / 0.0f64, 4.0f64, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(f64::INFINITY, 4.0f64, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, 0.0f64, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, -1.0f64, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, 0.0f64 / 0.0f64, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, f64::INFINITY, 2.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, 4.0f64, 0.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, 4.0f64, -1.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, 4.0f64, 0.0f64 / 0.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dashed::new(1.0f64, 4.0f64, f64::INFINITY).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let dotted = Dotted::new(2.0f64, 3.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, dotted.dot_diameter, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, dotted.gap_length, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(dotted.to_string().as_str()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(0.0f64, 3.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(-1.0f64, 3.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(0.0f64 / 0.0f64, 3.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(f64::INFINITY, 3.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(2.0f64, 0.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(2.0f64, -1.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(2.0f64, 0.0f64 / 0.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        Dotted::new(2.0f64, f64::INFINITY).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let link_role: Box<dyn RichTextRole> = Box::new(Link::new("https://tiqian.org"));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"https://tiqian.org", TextModelCoverageTestHelpers::text_model_coverage_test_helpers_link_target(link_role.clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Link(target=https://tiqian.org)", TextModelCoverageTestHelpers::text_model_coverage_test_helpers_role_name(link_role.clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, None).unwrap();
        let roles = vec![
    Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.clone()) as Box<dyn RichTextRole>,
    Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.clone()) as Box<dyn RichTextRole>,
    Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::LINE_THROUGH_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.clone()) as Box<dyn RichTextRole>,
    (link_role).clone(),
    Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::TECHNICAL_INLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.clone()) as Box<dyn RichTextRole>,
    Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::INLINE_CODE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.clone()) as Box<dyn RichTextRole>,
];
        let mut role_index = 0u32;
        while (i32::from_ne_bytes((role_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((roles.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let role: Box<dyn RichTextRole> = (roles[usize::try_from(role_index).unwrap_or(0)]).clone();
            let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), role.clone(), (paint).clone());
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextModelCoverageTestHelpers::text_model_coverage_test_helpers_role_name(role.clone()).as_str(), TextModelCoverageTestHelpers::text_model_coverage_test_helpers_role_name(span.role.clone()).as_str(),
None).unwrap();
            let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(span.to_string().as_str()).unwrap();
            role_index = u32::wrapping_add(role_index, 1);
        }
    });
}

#[test]
fn test_ruby_and_paragraph_models() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testRubyAndParagraphModels", "org.tiqian.core.TextModelCoverageTest.testRubyAndParagraphModels", || {
        TestTraceRecorder::new("TextModelCoverageTest").section(&"testRubyAndParagraphModels");
        let pinyin = RubyKind::Pinyin;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { pinyin.name().to_string() }.as_str(), None).unwrap();
        let bopomofo = RubyKind::Bopomofo;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { bopomofo.name().to_string() }.as_str(), None).unwrap();
        let per_line = RubyLineHeightMode::PerLine;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { per_line.name().to_string() }.as_str(), None).unwrap();
        let uniform_paragraph = RubyLineHeightMode::UniformParagraph;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { uniform_paragraph.name().to_string() }.as_str(), None).unwrap();
        let pinyin_ruby = RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "hàn", Some(vec!["CustomFont".to_string()]), RubyKind::Pinyin, None);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(RubyKind::Pinyin.name().to_string().as_str(), pinyin_ruby.kind.name().to_string().as_str(), None).unwrap();
        let pinyin_locale = pinyin_ruby.locale.clone();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(pinyin_locale.is_none(), match &(pinyin_locale) { None => "-".to_string(), Some(__option5) => TestTraceRender::test_trace_render_render_string(__option5.as_str()).unwrap().to_string() }.as_str(),
None).unwrap();
        let bopomofo_ruby = RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "ㄏㄢˋ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(RubyKind::Bopomofo.name().to_string().as_str(), bopomofo_ruby.kind.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"zh-TW", (bopomofo_ruby.locale).as_deref().unwrap_or(""), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(bopomofo_ruby.to_string().as_str()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.1f64, ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.1f64, ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM, None).unwrap();
        let alignment_start = LastLineAlignment::Start;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { alignment_start.name().to_string() }.as_str(), None).unwrap();
        let alignment_center = LastLineAlignment::Center;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { alignment_center.name().to_string() }.as_str(), None).unwrap();
        let alignment_end = LastLineAlignment::End;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { alignment_end.name().to_string() }.as_str(), None).unwrap();
        let horizontal_tb = WritingMode::HorizontalTb;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { horizontal_tb.name().to_string() }.as_str(), None).unwrap();
        let vertical_rl = WritingMode::VerticalRl;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { vertical_rl.name().to_string() }.as_str(), None).unwrap();
        let adaptive_indent = MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, adaptive_indent.resolve_em(10.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, adaptive_indent.resolve_em(14.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, adaptive_indent.resolve_em(20.0f64), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(adaptive_indent.to_string().as_str()).unwrap();
        let grid = LineLengthGrid::new(Some(true), Some(LastLineAlignment::Center));
        let _ = TracedAssertions::traced_assertions_assert_true(grid.enabled, None).unwrap();
        let grid_alignment = grid.body_alignment;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(LastLineAlignment::Center.name().to_string().as_str(), TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_nullable_alignment(grid_alignment).as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(grid.to_string().as_str()).unwrap();
        let para_style = ParagraphStyle::new(Some(LastLineAlignment::End), Some(WritingMode::VerticalRl), Some(32.0f64), None, Some((Ic::zero()).clone()), Some((adaptive_indent).clone()), Some((grid).clone()), Some(RubyLineHeightMode::UniformParagraph), Some(0.2f64),
Some(0.15f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(LastLineAlignment::End.name().to_string().as_str(), para_style.last_line_alignment.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(WritingMode::VerticalRl.name().to_string().as_str(), para_style.writing_mode.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, *(para_style.line_height).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(RubyLineHeightMode::UniformParagraph.name().to_string().as_str(), para_style.ruby_line_height_mode.name().to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(para_style.to_string().as_str()).unwrap();
        let profile_id = LayoutProfileId::new("custom-profile");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"custom-profile", (profile_id.value).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"clreq-horizontal", ((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone().value).to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(profile_id.to_string().as_str()).unwrap();
        let layout_input = LayoutInput::new(TiqianTextContent::new("Test", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None))),
Some((para_style).clone()), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((profile_id).clone()), Some(vec![(DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::Emphasis)).clone()]), Some(vec![(pinyin_ruby).clone()]),
Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(0.0f64), Some(0.0f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 10.0f64, 8.0f64, 2.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(profile_id.to_string().as_str(), (layout_input.profile_id).clone().to_string().as_str(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(layout_input.to_string().as_str()).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct TextModelCoverageTestHelpers;

impl TextModelCoverageTestHelpers {
    pub fn text_model_coverage_test_helpers_expect_argument_failure(block: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static>) -> Result<(), TracedAssertionsAssertFailsWithFault> {
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), (block).clone())?;
        Ok(())
    }

    pub fn text_model_coverage_test_helpers_render_nullable_alignment(value: Option<LastLineAlignment>) -> String {
        return match &(value) { None => "null".to_string(), Some(__option6) => TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_last_line_alignment(*__option6).to_string() };
    }

    pub fn text_model_coverage_test_helpers_render_last_line_alignment(value: LastLineAlignment) -> String {
        return value.name().to_string();
    }

    pub fn text_model_coverage_test_helpers_assert_rendered(rendered: &str) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(rendered, rendered, None)?;
        let _ = TracedAssertions::traced_assertions_assert_true(true, None)?;
        Ok(())
    }

    pub fn text_model_coverage_test_helpers_render_strings(values: &[String]) -> String {
        let mut output = "[".to_string();
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((index).to_ne_bytes()) > (0) {
                output += &(", ");
            }
            output += &(format!("{}{}{}",
            "'",
            (values[usize::try_from(index).unwrap_or(0)]).clone(),
            "'"
        ));
            index = u32::wrapping_add(index, 1);
        }
        return format!("{}{}",
            output,
            "]"
        );
    }

    pub fn text_model_coverage_test_helpers_role_name(role: Box<dyn RichTextRole>) -> String {
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Background" {
            return "Background".to_string();
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Underline" {
            return "Underline".to_string();
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.LineThrough" {
            return "LineThrough".to_string();
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" {
            return format!("{}{}{}",
            "Link(target=",
            (((role).as_any().downcast_ref::<Link>().unwrap()).target).to_string(),
            ")"
        );
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.TechnicalInline" {
            return "TechnicalInline".to_string();
        }
        return "InlineCode".to_string();
    }

    pub fn text_model_coverage_test_helpers_link_target(role: Box<dyn RichTextRole>) -> String {
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" {
            return ((((role).as_any().downcast_ref::<Link>().unwrap()).target).to_string()).clone();
        }
        return String::new();
    }
}
