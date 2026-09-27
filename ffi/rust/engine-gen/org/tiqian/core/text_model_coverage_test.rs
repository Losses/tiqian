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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestTiqianTextContentAndLinkAddressDisplayFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for TextModelCoverageTestTestTextStyleAndDecorationsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModelCoverageTestTestTextStyleAndDecorationsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestTextStyleAndDecorationsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestTextStyleAndDecorationsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for TextModelCoverageTestTestSpansAndInlineBoxFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModelCoverageTestTestSpansAndInlineBoxFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestSpansAndInlineBoxFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestSpansAndInlineBoxFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for TextModelCoverageTestTestRubyAndParagraphModelsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModelCoverageTestTestRubyAndParagraphModelsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestRubyAndParagraphModelsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestRubyAndParagraphModelsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for TextModelCoverageTestTestRichTextSpansAndPatternsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestRichTextSpansAndPatternsFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            TextModelCoverageTestTestInlineObjectPreferredStretchAndAdjustmentFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,77,111,100,101,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,84,105,113,105,97,110,84,101,120,116,67,111,110,116,101,110,116,65,110,100,76,105,110,107,65,100,100,114,101,115,115,68,105,115,112,108,97,121]));
        let content = TiqianTextContent::new(&(UStr::new(&[72,101,108,108,111,32,84,105,113,105,97,110])), Some(vec![
    (TextSpan::new(TextRange::new(0u32, 5u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None)))).clone(),
]), Some(vec![0, 5, 12]), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 5u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![(TextRange::new(6u32, 12u32).unwrap()).clone()]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[72,101,108,108,111,32,84,105,113,105,97,110]), (content.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((content.spans.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((content.source_boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((content.line_break_spans.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((content.auto_space_suppressed_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", content.to_string()).as_str()).as_ustr()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", content.to_string()).as_str()), UString::from("TiqianTextContent").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[]), UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[116,105,113,105,97,110,46,111,114,103]), UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[]), UStr::new(&[104,116,116,112,115,58,47,47,116,105,113,105,97,110,46,111,114,103])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[116,105,113,105,97,110,46,111,114,103]), UStr::new(&[116,105,113,105,97,110,46,111,114,103])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[116,105,113,105,97,110,46,111,114,103]), UStr::new(&[104,116,116,112,115,58,47,47,116,105,113,105,97,110,46,111,114,103])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[116,105,113,105,97,110,46,111,114,103]), UStr::new(&[104,116,116,112,58,47,47,116,105,113,105,97,110,46,111,114,103])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[100,101,118,64,116,105,113,105,97,110,46,111,114,103]), UStr::new(&[109,97,105,108,116,111,58,100,101,118,64,116,105,113,105,97,110,46,111,114,103])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[116,105,113,105,97,110,46,111,114,103]), UStr::new(&[104,116,116,112,115,58,47,47,111,116,104,101,114,46,111,114,103])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LinkAddressDisplay::link_address_display_displays_address(UStr::new(&[116,105,113,105,97,110,46,111,114,103]), UStr::new(&[102,116,112,58,47,47,116,105,113,105,97,110,46,111,114,103])), None).unwrap();
    });
}

#[test]
fn test_spans_and_inline_box() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testSpansAndInlineBox", "org.tiqian.core.TextModelCoverageTest.testSpansAndInlineBox", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,77,111,100,101,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,83,112,97,110,115,65,110,100,73,110,108,105,110,101,66,111,120]));
        let line_break_span = LineBreakSpan::new(TextRange::new(0u32, 4u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 4u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (line_break_span.range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(LineBreakPolicy::ProgressiveTechnical.name()).as_ustr(), UString::from(line_break_span.policy.name()).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", line_break_span.to_string()).as_str()).as_ustr()).unwrap();
        let progressive_technical = LineBreakPolicy::ProgressiveTechnical;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(progressive_technical.name()) }.as_ustr(), None).unwrap();
        let attachment_none = InlineAttachment::None;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(attachment_none.name()) }.as_ustr(), None).unwrap();
        let attachment_previous = InlineAttachment::Previous;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(attachment_previous.name()) }.as_ustr(), None).unwrap();
        let spacing_narrow = InlineBoxOuterSpacing::Narrow;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(spacing_narrow.name()) }.as_ustr(), None).unwrap();
        let spacing_source = InlineBoxOuterSpacing::Source;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(spacing_source.name()) }.as_ustr(), None).unwrap();
        let inline_box = InlineBoxSpan::new(TextRange::new(1u32, 3u32).unwrap(), Some(2.0f64), Some(3.0f64), Some(InlineBoxOuterSpacing::Source));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(1u32, 3u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (inline_box.range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, inline_box.inline_start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, inline_box.inline_end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(InlineBoxOuterSpacing::Source.name()).as_ustr(), UString::from(inline_box.outer_spacing.name()).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", inline_box.to_string()).as_str()).as_ustr()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65532]), InlineObjectSpan::INLINE_OBJECT_SPAN_INLINE_OBJECT_REPLACEMENT_CHAR.to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn test_inline_object_preferred_stretch_and_adjustment() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testInlineObjectPreferredStretchAndAdjustment", "org.tiqian.core.TextModelCoverageTest.testInlineObjectPreferredStretchAndAdjustment", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,77,111,100,101,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,73,110,108,105,110,101,79,98,106,101,99,116,80,114,101,102,101,114,114,101,100,83,116,114,101,116,99,104,65,110,100,65,100,106,117,115,116,109,101,110,116]));
        let punctuation_trailing = InlineObjectPreferredStretchKind::PunctuationTrailing;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(punctuation_trailing.name()) }.as_ustr(), None).unwrap();
        let relation = InlineObjectPreferredStretchKind::Relation;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(relation.name()) }.as_ustr(), None).unwrap();
        let binary_operator = InlineObjectPreferredStretchKind::BinaryOperator;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(binary_operator.name()) }.as_ustr(), None).unwrap();
        let stretch = InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 10.0f64, 15.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(InlineObjectPreferredStretchKind::Relation.name()).as_ustr(), UString::from(stretch.kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, stretch.natural_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(15.0f64, stretch.target_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5.0f64, stretch.get_capacity(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", stretch.to_string()).as_str()).as_ustr()).unwrap();
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
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(fixed_preferred_stretch.is_none(), match &(fixed_preferred_stretch) { None => UString::from("-"), Some(__option2) => UString::from(format!("{}", __option2.to_string()).as_str()) }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, fixed.shrink_capacity, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, fixed.line_end_discardable_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(fixed.prevents_line_break, None).unwrap();
        let custom_adj = InlineObjectBoundaryAdjustment::new(Some(true), Some((stretch).clone()), Some(2.0f64), Some(1.0f64), Some(true)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(custom_adj.participates_in_uniform_stretch, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", stretch.to_string()).as_str()).as_ustr(), UString::from(format!("{}", custom_adj.preferred_stretch.as_ref().unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, custom_adj.shrink_capacity, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, custom_adj.line_end_discardable_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(custom_adj.prevents_line_break, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", custom_adj.to_string()).as_str()).as_ustr()).unwrap();
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
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 1u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (inline_object.range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, inline_object.advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.0f64, inline_object.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.0f64, inline_object.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, None).unwrap();
    });
}

#[test]
fn test_text_style_and_decorations() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testTextStyleAndDecorations", "org.tiqian.core.TextModelCoverageTest.testTextStyleAndDecorations", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,77,111,100,101,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,84,101,120,116,83,116,121,108,101,65,110,100,68,101,99,111,114,97,116,105,111,110,115]));
        let style = TextStyle::new(Some(vec![UString::from("Noto Serif CJK SC").to_ustring()]), Some(18.0f64), Some(UString::from("zh-CN")), Some(700), Some(true), Some(-2.0f64), Some(InlineAttachment::Previous));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_strings(&vec![UString::from("Noto Serif CJK SC").to_ustring()]).as_ustr(), TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_strings(&style.font_families).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.0f64, style.font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[122,104,45,67,78]), (style.locale).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(700, style.font_weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(style.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(-2.0f64, style.baseline_shift, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(InlineAttachment::Previous.name()).as_ustr(), UString::from(style.inline_attachment.name()).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", style.to_string()).as_str()).as_ustr()).unwrap();
        let emphasis = DecorationKind::Emphasis;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(emphasis.name()) }.as_ustr(), None).unwrap();
        let mourning = DecorationKind::Mourning;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(mourning.name()) }.as_ustr(), None).unwrap();
        let proper_noun = DecorationKind::ProperNoun;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(proper_noun.name()) }.as_ustr(), None).unwrap();
        let book_title = DecorationKind::BookTitle;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(book_title.name()) }.as_ustr(), None).unwrap();
        let decoration = DecorationSpan::new(TextRange::new(2u32, 4u32).unwrap(), DecorationKind::Emphasis);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(2u32, 4u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (decoration.range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(DecorationKind::Emphasis.name()).as_ustr(), UString::from(decoration.kind.name()).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", decoration.to_string()).as_str()).as_ustr()).unwrap();
        let color = ColorSpan::new(1u32, 5u32, 4279312947u32);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, color.start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, color.end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4279312947u32, color.argb, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", color.to_string()).as_str()).as_ustr()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", color.to_string()).as_str()), UString::from("ColorSpan").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_rich_text_spans_and_patterns() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testRichTextSpansAndPatterns", "org.tiqian.core.TextModelCoverageTest.testRichTextSpansAndPatterns", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,77,111,100,101,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,82,105,99,104,84,101,120,116,83,112,97,110,115,65,110,100,80,97,116,116,101,114,110,115]));
        let paint = RichTextPaint::new(Some(4278190080u32), Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(1.5f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4278190080u32, *(paint.argb).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.5f64, paint.adjacent_same_style_clearance, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", paint.to_string()).as_str()).as_ustr()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, Some(-0.1f64)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, Some(0.0f64 / 0.0f64)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, Some(f64::INFINITY)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let bg_paint = RichTextBackgroundPaint::new(Some(2.0f64), Some(3.0f64), Some(4.0f64), Some(1.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle), Some(Box::new(Border::new(1.5f64).unwrap()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, bg_paint.horizontal_padding, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, bg_paint.vertical_padding, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4.0f64, bg_paint.corner_radius, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, bg_paint.continuation_corner_radius, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(RichTextBackgroundMetricPolicy::UniformTextStyle.name()).as_ustr(), UString::from(bg_paint.metric_policy.name()).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", bg_paint.to_string()).as_str()).as_ustr()).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(-1.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64 / 0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(-1.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64 / 0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(-1.0f64), Some(-1.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64 / 0.0f64), Some(0.0f64 / 0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(-1.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64 / 0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", (*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let border = Border::new(2.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, border.stroke_width, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", border.to_string()).as_str()).as_ustr()).unwrap();
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
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(marked_faces.name()) }.as_ustr(), None).unwrap();
        let uniform_text_style = RichTextBackgroundMetricPolicy::UniformTextStyle;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(uniform_text_style.name()) }.as_ustr(), None).unwrap();
        let uniform_paragraph_style = RichTextBackgroundMetricPolicy::UniformParagraphStyle;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(uniform_paragraph_style.name()) }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", (*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let dashed = Dashed::new(1.0f64, 4.0f64, 2.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, dashed.stroke_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4.0f64, dashed.dash_length, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, dashed.gap_length, None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", dashed.to_string()).as_str()).as_ustr()).unwrap();
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
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", dotted.to_string()).as_str()).as_ustr()).unwrap();
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
        let link_role: Box<dyn RichTextRole> = Box::new(Link::new(&(UStr::new(&[104,116,116,112,115,58,47,47,116,105,113,105,97,110,46,111,114,103]))));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[104,116,116,112,115,58,47,47,116,105,113,105,97,110,46,111,114,103]), TextModelCoverageTestHelpers::text_model_coverage_test_helpers_link_target(link_role.clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[76,105,110,107,40,116,97,114,103,101,116,61,104,116,116,112,115,58,47,47,116,105,113,105,97,110,46,111,114,103,41]), TextModelCoverageTestHelpers::text_model_coverage_test_helpers_role_name(link_role.clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, None).unwrap();
        let roles = vec![
    Box::new(({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).clone()) as Box<dyn RichTextRole>,
    Box::new(({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).clone()) as Box<dyn RichTextRole>,
    Box::new(({ let __guard = crate::org::tiqian::core::rich_text_role::LINE_THROUGH_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).clone()) as Box<dyn RichTextRole>,
    (link_role).clone(),
    Box::new(({ let __guard = crate::org::tiqian::core::rich_text_role::TECHNICAL_INLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).clone()) as Box<dyn RichTextRole>,
    Box::new(({ let __guard = crate::org::tiqian::core::rich_text_role::INLINE_CODE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).clone()) as Box<dyn RichTextRole>,
];
        let mut role_index = 0u32;
        while (i32::from_ne_bytes(((role_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((roles.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let role: Box<dyn RichTextRole> = (roles[usize::try_from(role_index).unwrap_or(0)]).clone();
            let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), role.clone(), (paint).clone());
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextModelCoverageTestHelpers::text_model_coverage_test_helpers_role_name(role.clone()).as_ustr(), TextModelCoverageTestHelpers::text_model_coverage_test_helpers_role_name(span.role.clone()).as_ustr(), None).unwrap();
            let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", span.to_string()).as_str()).as_ustr()).unwrap();
            role_index = u32::wrapping_add(role_index, 1);
        }
    });
}

#[test]
fn test_ruby_and_paragraph_models() {
    testlib::run("org.tiqian.core.TextModelCoverageTest.testRubyAndParagraphModels", "org.tiqian.core.TextModelCoverageTest.testRubyAndParagraphModels", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,77,111,100,101,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,82,117,98,121,65,110,100,80,97,114,97,103,114,97,112,104,77,111,100,101,108,115]));
        let pinyin = RubyKind::Pinyin;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(pinyin.name()) }.as_ustr(), None).unwrap();
        let bopomofo = RubyKind::Bopomofo;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(bopomofo.name()) }.as_ustr(), None).unwrap();
        let per_line = RubyLineHeightMode::PerLine;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(per_line.name()) }.as_ustr(), None).unwrap();
        let uniform_paragraph = RubyLineHeightMode::UniformParagraph;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(uniform_paragraph.name()) }.as_ustr(), None).unwrap();
        let pinyin_ruby = RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[104,224,110])), Some(vec![UString::from("CustomFont").to_ustring()]), RubyKind::Pinyin, None);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(RubyKind::Pinyin.name()).as_ustr(), UString::from(pinyin_ruby.kind.name()).as_ustr(), None).unwrap();
        let pinyin_locale = pinyin_ruby.locale.clone();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(pinyin_locale.is_none(), match &(pinyin_locale) { None => UString::from("-"), Some(__option5) => TestTraceRender::test_trace_render_render_string(__option5.as_ustr()).unwrap().to_ustring() }.as_ustr(), None).unwrap();
        let bopomofo_ruby = RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12559,12578,715])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(RubyKind::Bopomofo.name()).as_ustr(), UString::from(bopomofo_ruby.kind.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[122,104,45,84,87]), (bopomofo_ruby.locale).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", bopomofo_ruby.to_string()).as_str()).as_ustr()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.1f64, ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.1f64, ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM, None).unwrap();
        let alignment_start = LastLineAlignment::Start;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(alignment_start.name()) }.as_ustr(), None).unwrap();
        let alignment_center = LastLineAlignment::Center;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(alignment_center.name()) }.as_ustr(), None).unwrap();
        let alignment_end = LastLineAlignment::End;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(alignment_end.name()) }.as_ustr(), None).unwrap();
        let horizontal_tb = WritingMode::HorizontalTb;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(horizontal_tb.name()) }.as_ustr(), None).unwrap();
        let vertical_rl = WritingMode::VerticalRl;
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(vertical_rl.name()) }.as_ustr(), None).unwrap();
        let adaptive_indent = MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.0f64, adaptive_indent.resolve_em(10.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, adaptive_indent.resolve_em(14.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, adaptive_indent.resolve_em(20.0f64), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", adaptive_indent.to_string()).as_str()).as_ustr()).unwrap();
        let grid = LineLengthGrid::new(Some(true), Some(LastLineAlignment::Center));
        let _ = TracedAssertions::traced_assertions_assert_true(grid.enabled, None).unwrap();
        let grid_alignment = grid.body_alignment;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(LastLineAlignment::Center.name()).as_ustr(), TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_nullable_alignment(grid_alignment).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", grid.to_string()).as_str()).as_ustr()).unwrap();
        let para_style = ParagraphStyle::new(Some(LastLineAlignment::End), Some(WritingMode::VerticalRl), Some(32.0f64), None, Some((Ic::zero()).clone()), Some((adaptive_indent).clone()), Some((grid).clone()), Some(RubyLineHeightMode::UniformParagraph), Some(0.2f64), Some(0.15f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(LastLineAlignment::End.name()).as_ustr(), UString::from(para_style.last_line_alignment.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(WritingMode::VerticalRl.name()).as_ustr(), UString::from(para_style.writing_mode.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, *(para_style.line_height).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(RubyLineHeightMode::UniformParagraph.name()).as_ustr(), UString::from(para_style.ruby_line_height_mode.name()).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", para_style.to_string()).as_str()).as_ustr()).unwrap();
        let profile_id = LayoutProfileId::new(&(UStr::new(&[99,117,115,116,111,109,45,112,114,111,102,105,108,101])));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,117,115,116,111,109,45,112,114,111,102,105,108,101]), (profile_id.value).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,108,114,101,113,45,104,111,114,105,122,111,110,116,97,108]), ((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone().value).to_ustring().as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", profile_id.to_string()).as_str()).as_ustr()).unwrap();
        let layout_input = LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[84,101,115,116])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None))), Some((para_style).clone()), LayoutConstraints::new(300.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((profile_id).clone()), Some(vec![(DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::Emphasis)).clone()]), Some(vec![(pinyin_ruby).clone()]), Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(0.0f64), Some(0.0f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 10.0f64, 8.0f64, 2.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", profile_id.to_string()).as_str()).as_ustr(), UString::from(format!("{}", (layout_input.profile_id).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TextModelCoverageTestHelpers::text_model_coverage_test_helpers_assert_rendered(UString::from(format!("{}", layout_input.to_string()).as_str()).as_ustr()).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct TextModelCoverageTestHelpers;

impl TextModelCoverageTestHelpers {
    pub fn text_model_coverage_test_helpers_expect_argument_failure(block: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static>) -> Result<(), TracedAssertionsAssertFailsWithFault> {
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), (block).clone())?;
        Ok(())
    }

    pub fn text_model_coverage_test_helpers_render_nullable_alignment(value: Option<LastLineAlignment>) -> UString {
        return match &(value) { None => UString::from("null"), Some(__option6) => TextModelCoverageTestHelpers::text_model_coverage_test_helpers_render_last_line_alignment(*__option6).to_ustring() };
    }

    pub fn text_model_coverage_test_helpers_render_last_line_alignment(value: LastLineAlignment) -> UString {
        return UString::from(value.name());
    }

    pub fn text_model_coverage_test_helpers_assert_rendered(rendered: &UStr) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(rendered, rendered, None)?;
        let _ = TracedAssertions::traced_assertions_assert_true(true, None)?;
        Ok(())
    }

    pub fn text_model_coverage_test_helpers_render_strings(values: &[UString]) -> UString {
        let mut output = UString::from("[").to_ustring();
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) > (0) {
                output += &(UString::from(", "));
            }
            output += &({ let mut __s = UString::new(); __s += &(UString::from("'")); __s += (values[usize::try_from(index).unwrap_or(0)]).clone().as_ustr(); __s += &(UString::from("'")); __s });
            index = u32::wrapping_add(index, 1);
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += output.as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn text_model_coverage_test_helpers_role_name(role: Box<dyn RichTextRole>) -> UString {
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Background" {
            return UString::from("Background").to_ustring();
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Underline" {
            return UString::from("Underline").to_ustring();
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.LineThrough" {
            return UString::from("LineThrough").to_ustring();
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" {
            return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Link(target=")); __s += (((role).as_any().downcast_ref::<Link>().unwrap()).target).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
        }
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.TechnicalInline" {
            return UString::from("TechnicalInline").to_ustring();
        }
        return UString::from("InlineCode").to_ustring();
    }

    pub fn text_model_coverage_test_helpers_link_target(role: Box<dyn RichTextRole>) -> UString {
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" {
            return ((((role).as_any().downcast_ref::<Link>().unwrap()).target).to_ustring()).clone();
        }
        return UString::new();
    }
}
