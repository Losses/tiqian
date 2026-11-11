use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::bopomofo_decision_info::BopomofoDecisionInfo;
use crate::org::tiqian::core::break_opportunity_decision_info::BreakOpportunityDecisionInfo;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::contextual_kinsoku_decision_info::ContextualKinsokuDecisionInfo;
use crate::org::tiqian::core::decoration_decision_info::DecorationDecisionInfo;
use crate::org::tiqian::core::decoration_segment_info::DecorationSegmentInfo;
use crate::org::tiqian::core::emergency_tracking_eligibility_decision_info::EmergencyTrackingEligibilityDecisionInfo;
use crate::org::tiqian::core::first_line_indent_decision_info::FirstLineIndentDecisionInfo;
use crate::org::tiqian::core::font_decision_info::FontDecisionInfo;
use crate::org::tiqian::core::inline_box_decision_info::InlineBoxDecisionInfo;
use crate::org::tiqian::core::inline_object_decision_info::InlineObjectDecisionInfo;
use crate::org::tiqian::core::inline_object_line_height_decision_info::InlineObjectLineHeightDecisionInfo;
use crate::org::tiqian::core::inline_object_punctuation_attachment_decision_info::InlineObjectPunctuationAttachmentDecisionInfo;
use crate::org::tiqian::core::justification_decision_info::JustificationDecisionInfo;
use crate::org::tiqian::core::kinsoku_decision_info::KinsokuDecisionInfo;
use crate::org::tiqian::core::line_decision_info::LineDecisionInfo;
use crate::org::tiqian::core::line_edge_trim_decision_info::LineEdgeTrimDecisionInfo;
use crate::org::tiqian::core::line_length_grid_decision_info::LineLengthGridDecisionInfo;
use crate::org::tiqian::core::line_spacing_decision_info::LineSpacingDecisionInfo;
use crate::org::tiqian::core::mandatory_break_decision_info::MandatoryBreakDecisionInfo;
use crate::org::tiqian::core::max_lines_decision_info::MaxLinesDecisionInfo;
use crate::org::tiqian::core::metric_decision_info::MetricDecisionInfo;
use crate::org::tiqian::core::punctuation_decision_info::PunctuationDecisionInfo;
use crate::org::tiqian::core::role_override_info::RoleOverrideInfo;
use crate::org::tiqian::core::ruby_decision_info::RubyDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_decision_info::RubyLineHeightDecisionInfo;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::spacing_decision_info::SpacingDecisionInfo;
use crate::org::tiqian::core::zero_width_break_decision_info::ZeroWidthBreakDecisionInfo;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutDebugInfo {
    pub font_decisions: Vec<FontDecisionInfo>,
    pub shaping_decisions: Vec<ShapingDecisionInfo>,
    pub metric_decisions: Vec<MetricDecisionInfo>,
    pub punctuation_decisions: Vec<PunctuationDecisionInfo>,
    pub geometry_decisions: Vec<ClusterGeometryDecisionInfo>,
    pub spacing_decisions: Vec<SpacingDecisionInfo>,
    pub role_overrides: Vec<RoleOverrideInfo>,
    pub line_decisions: Vec<LineDecisionInfo>,
    pub justification_decisions: Vec<JustificationDecisionInfo>,
    pub auto_space_decisions: Vec<AutoSpaceDecisionInfo>,
    pub line_edge_trim_decisions: Vec<LineEdgeTrimDecisionInfo>,
    pub decoration_decisions: Vec<DecorationDecisionInfo>,
    pub decoration_segments: Vec<DecorationSegmentInfo>,
    pub ruby_decisions: Vec<RubyDecisionInfo>,
    pub bopomofo_decisions: Vec<BopomofoDecisionInfo>,
    pub mandatory_break_decisions: Vec<MandatoryBreakDecisionInfo>,
    pub max_lines_decision: Option<MaxLinesDecisionInfo>,
    pub line_spacing_decision: Option<LineSpacingDecisionInfo>,
    pub ruby_line_height_decision: Option<RubyLineHeightDecisionInfo>,
    pub inline_object_line_height_decision: Option<InlineObjectLineHeightDecisionInfo>,
    pub kinsoku_decision: Option<KinsokuDecisionInfo>,
    pub contextual_kinsoku_decisions: Vec<ContextualKinsokuDecisionInfo>,
    pub line_length_grid_decision: Option<LineLengthGridDecisionInfo>,
    pub first_line_indent_decision: Option<FirstLineIndentDecisionInfo>,
    pub inline_box_decisions: Vec<InlineBoxDecisionInfo>,
    pub inline_object_decisions: Vec<InlineObjectDecisionInfo>,
    pub inline_object_punctuation_attachment_decisions: Vec<InlineObjectPunctuationAttachmentDecisionInfo>,
    pub zero_width_break_decisions: Vec<ZeroWidthBreakDecisionInfo>,
    pub break_opportunity_decisions: Vec<BreakOpportunityDecisionInfo>,
    pub emergency_tracking_eligibility_decisions: Vec<EmergencyTrackingEligibilityDecisionInfo>,
}

impl LayoutDebugInfo {
    pub fn new(max_lines_decision: Option<MaxLinesDecisionInfo>, metric_decisions: Option<Vec<MetricDecisionInfo>>, geometry_decisions: Option<Vec<ClusterGeometryDecisionInfo>>, auto_space_decisions: Option<Vec<AutoSpaceDecisionInfo>>, ruby_decisions: Option<Vec<RubyDecisionInfo>>, bopomofo_decisions: Option<Vec<BopomofoDecisionInfo>>, font_decisions: Option<Vec<FontDecisionInfo>>, shaping_decisions: Option<Vec<ShapingDecisionInfo>>, punctuation_decisions: Option<Vec<PunctuationDecisionInfo>>, spacing_decisions: Option<Vec<SpacingDecisionInfo>>, role_overrides: Option<Vec<RoleOverrideInfo>>, line_decisions: Option<Vec<LineDecisionInfo>>, justification_decisions: Option<Vec<JustificationDecisionInfo>>, line_edge_trim_decisions: Option<Vec<LineEdgeTrimDecisionInfo>>, decoration_decisions: Option<Vec<DecorationDecisionInfo>>, decoration_segments: Option<Vec<DecorationSegmentInfo>>, mandatory_break_decisions: Option<Vec<MandatoryBreakDecisionInfo>>, line_spacing_decision: Option<LineSpacingDecisionInfo>, ruby_line_height_decision: Option<RubyLineHeightDecisionInfo>, inline_object_line_height_decision: Option<InlineObjectLineHeightDecisionInfo>, kinsoku_decision: Option<KinsokuDecisionInfo>, contextual_kinsoku_decisions: Option<Vec<ContextualKinsokuDecisionInfo>>, line_length_grid_decision: Option<LineLengthGridDecisionInfo>, first_line_indent_decision: Option<FirstLineIndentDecisionInfo>, inline_box_decisions: Option<Vec<InlineBoxDecisionInfo>>, inline_object_decisions: Option<Vec<InlineObjectDecisionInfo>>, inline_object_punctuation_attachment_decisions: Option<Vec<InlineObjectPunctuationAttachmentDecisionInfo>>, zero_width_break_decisions: Option<Vec<ZeroWidthBreakDecisionInfo>>, break_opportunity_decisions: Option<Vec<BreakOpportunityDecisionInfo>>, emergency_tracking_eligibility_decisions: Option<Vec<EmergencyTrackingEligibilityDecisionInfo>>) -> Self {
        let metric_decisions = metric_decisions.unwrap_or_else(|| vec![]);
        let geometry_decisions = geometry_decisions.unwrap_or_else(|| vec![]);
        let auto_space_decisions = auto_space_decisions.unwrap_or_else(|| vec![]);
        let ruby_decisions = ruby_decisions.unwrap_or_else(|| vec![]);
        let bopomofo_decisions = bopomofo_decisions.unwrap_or_else(|| vec![]);
        let font_decisions = font_decisions.unwrap_or_else(|| vec![]);
        let shaping_decisions = shaping_decisions.unwrap_or_else(|| vec![]);
        let punctuation_decisions = punctuation_decisions.unwrap_or_else(|| vec![]);
        let spacing_decisions = spacing_decisions.unwrap_or_else(|| vec![]);
        let role_overrides = role_overrides.unwrap_or_else(|| vec![]);
        let line_decisions = line_decisions.unwrap_or_else(|| vec![]);
        let justification_decisions = justification_decisions.unwrap_or_else(|| vec![]);
        let line_edge_trim_decisions = line_edge_trim_decisions.unwrap_or_else(|| vec![]);
        let decoration_decisions = decoration_decisions.unwrap_or_else(|| vec![]);
        let decoration_segments = decoration_segments.unwrap_or_else(|| vec![]);
        let mandatory_break_decisions = mandatory_break_decisions.unwrap_or_else(|| vec![]);
        let contextual_kinsoku_decisions = contextual_kinsoku_decisions.unwrap_or_else(|| vec![]);
        let inline_box_decisions = inline_box_decisions.unwrap_or_else(|| vec![]);
        let inline_object_decisions = inline_object_decisions.unwrap_or_else(|| vec![]);
        let inline_object_punctuation_attachment_decisions = inline_object_punctuation_attachment_decisions.unwrap_or_else(|| vec![]);
        let zero_width_break_decisions = zero_width_break_decisions.unwrap_or_else(|| vec![]);
        let break_opportunity_decisions = break_opportunity_decisions.unwrap_or_else(|| vec![]);
        let emergency_tracking_eligibility_decisions = emergency_tracking_eligibility_decisions.unwrap_or_else(|| vec![]);
        Self {
            max_lines_decision,
            metric_decisions: metric_decisions,
            geometry_decisions: geometry_decisions,
            auto_space_decisions: auto_space_decisions,
            ruby_decisions: ruby_decisions,
            bopomofo_decisions: bopomofo_decisions,
            font_decisions: font_decisions,
            shaping_decisions: shaping_decisions,
            punctuation_decisions: punctuation_decisions,
            spacing_decisions: spacing_decisions,
            role_overrides: role_overrides,
            line_decisions: line_decisions,
            justification_decisions: justification_decisions,
            line_edge_trim_decisions: line_edge_trim_decisions,
            decoration_decisions: decoration_decisions,
            decoration_segments: decoration_segments,
            mandatory_break_decisions: mandatory_break_decisions,
            line_spacing_decision,
            ruby_line_height_decision,
            inline_object_line_height_decision,
            kinsoku_decision,
            contextual_kinsoku_decisions: contextual_kinsoku_decisions,
            line_length_grid_decision,
            first_line_indent_decision,
            inline_box_decisions: inline_box_decisions,
            inline_object_decisions: inline_object_decisions,
            inline_object_punctuation_attachment_decisions: inline_object_punctuation_attachment_decisions,
            zero_width_break_decisions: zero_width_break_decisions,
            break_opportunity_decisions: break_opportunity_decisions,
            emergency_tracking_eligibility_decisions: emergency_tracking_eligibility_decisions,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LayoutDebugInfo(")); __s += &(UString::from("fontDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.font_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("shapingDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.shaping_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("metricDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.metric_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.punctuation_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("geometryDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.geometry_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("spacingDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.spacing_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("roleOverrides=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.role_overrides).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("justificationDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.justification_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("autoSpaceDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.auto_space_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineEdgeTrimDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_edge_trim_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("decorationDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decoration_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("decorationSegments=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decoration_segments).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rubyDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.ruby_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("bopomofoDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.bopomofo_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("mandatoryBreakDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.mandatory_break_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("maxLinesDecision=")); __s += (match &(self.max_lines_decision) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineSpacingDecision=")); __s += (match &(self.line_spacing_decision) { None => UString::from("null"), Some(__option1) => UString::from(format!("{}", __option1.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rubyLineHeightDecision=")); __s += (match &(self.ruby_line_height_decision) { None => UString::from("null"), Some(__option2) => UString::from(format!("{}", __option2.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineObjectLineHeightDecision=")); __s += (match &(self.inline_object_line_height_decision) { None => UString::from("null"), Some(__option3) => UString::from(format!("{}", __option3.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("kinsokuDecision=")); __s += (match &(self.kinsoku_decision) { None => UString::from("null"), Some(__option4) => UString::from(format!("{}", __option4.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("contextualKinsokuDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.contextual_kinsoku_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineLengthGridDecision=")); __s += (match &(self.line_length_grid_decision) { None => UString::from("null"), Some(__option5) => UString::from(format!("{}", __option5.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("firstLineIndentDecision=")); __s += (match &(self.first_line_indent_decision) { None => UString::from("null"), Some(__option6) => UString::from(format!("{}", __option6.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineBoxDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_box_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineObjectDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_object_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineObjectPunctuationAttachmentDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.inline_object_punctuation_attachment_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("zeroWidthBreakDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.zero_width_break_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("breakOpportunityDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.break_opportunity_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("emergencyTrackingEligibilityDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.emergency_tracking_eligibility_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn layout_debug_info_with_metric_decisions(values: &Vec<MetricDecisionInfo>) -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some((values).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn layout_debug_info_with_geometry_decisions(values: &Vec<ClusterGeometryDecisionInfo>) -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some((values).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn layout_debug_info_with_auto_space_decisions(values: &Vec<AutoSpaceDecisionInfo>) -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some((values).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn layout_debug_info_with_ruby_decisions(values: &Vec<RubyDecisionInfo>) -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some((values).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn layout_debug_info_with_bopomofo_decisions(values: &Vec<BopomofoDecisionInfo>) -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some((values).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }
}
