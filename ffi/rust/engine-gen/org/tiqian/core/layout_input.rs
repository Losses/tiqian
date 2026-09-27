use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutInput {
    pub content: TiqianTextContent,
    pub text_style: TextStyle,
    pub paragraph_style: ParagraphStyle,
    pub constraints: LayoutConstraints,
    pub profile_id: LayoutProfileId,
    pub decorations: Vec<DecorationSpan>,
    pub ruby_spans: Vec<RubySpan>,
    pub inline_boxes: Vec<InlineBoxSpan>,
    pub inline_objects: Vec<InlineObjectSpan>,
}

impl LayoutInput {
    pub fn new(content: TiqianTextContent, text_style: Option<TextStyle>, paragraph_style: Option<ParagraphStyle>, constraints: LayoutConstraints, profile_id: Option<LayoutProfileId>, decorations: Option<Vec<DecorationSpan>>, ruby_spans: Option<Vec<RubySpan>>, inline_boxes:
Option<Vec<InlineBoxSpan>>, inline_objects: Option<Vec<InlineObjectSpan>>) -> Self {
        let text_style = text_style.unwrap_or_else(|| TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)));
        let paragraph_style = paragraph_style.unwrap_or_else(|| ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM)));
        let profile_id = profile_id.unwrap_or_else(|| (*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone());
        let decorations = decorations.unwrap_or_else(|| vec![]);
        let ruby_spans = ruby_spans.unwrap_or_else(|| vec![]);
        let inline_boxes = inline_boxes.unwrap_or_else(|| vec![]);
        let inline_objects = inline_objects.unwrap_or_else(|| vec![]);
        Self {
            content,
            text_style: text_style,
            paragraph_style: paragraph_style,
            constraints,
            profile_id: profile_id,
            decorations: decorations,
            ruby_spans: ruby_spans,
            inline_boxes: inline_boxes,
            inline_objects: inline_objects,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LayoutInput(",
            "content=",
            (self.content).clone().to_string(),
            ", ",
            "textStyle=",
            (self.text_style).clone().to_string(),
            ", ",
            "paragraphStyle=",
            (self.paragraph_style).clone().to_string(),
            ", ",
            "constraints=",
            (self.constraints).clone().to_string(),
            ", ",
            "profileId=",
            (self.profile_id).clone().to_string(),
            ", ",
            "decorations=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decorations).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "rubySpans=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.ruby_spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "inlineBoxes=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_boxes).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "inlineObjects=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_objects).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ")"
        );
    }
}
