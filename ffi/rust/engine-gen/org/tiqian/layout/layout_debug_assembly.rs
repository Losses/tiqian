use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::bopomofo_decision_info::BopomofoDecisionInfo;
use crate::org::tiqian::core::break_opportunity_decision_info::BreakOpportunityDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
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
use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::justification_decision_info::JustificationDecisionInfo;
use crate::org::tiqian::core::kinsoku_decision_info::KinsokuDecisionInfo;
use crate::org::tiqian::core::layout_debug_info::LayoutDebugInfo;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_decision_info::LineDecisionInfo;
use crate::org::tiqian::core::line_edge_trim_decision_info::LineEdgeTrimDecisionInfo;
use crate::org::tiqian::core::line_length_grid_decision_info::LineLengthGridDecisionInfo;
use crate::org::tiqian::core::line_repair_allocation_info::LineRepairAllocationInfo;
use crate::org::tiqian::core::line_repair_candidate_info::LineRepairCandidateInfo;
use crate::org::tiqian::core::line_repair_decision_info::LineRepairDecisionInfo;
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
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::zero_width_break_decision_info::ZeroWidthBreakDecisionInfo;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::layout::contextual_punctuation_display_substitution::ContextualPunctuationDisplaySubstitutionFns;
use crate::org::tiqian::layout::justifier::JustificationPlan;
use crate::org::tiqian::layout::line_geometry_stage::ClusterMetricDecision;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::line_optimization::RepairCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::punctuation_geometry_ledger::AttachedInlinePunctuationBoundaryResult;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressionResult;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;


#[derive(Clone, PartialEq)]
pub struct LayoutDebugStageInput {
    pub text: UString,
    pub font_decisions: Vec<FontDecision>,
    pub punctuation_glyph_substitutor: ClreqPunctuationGlyphSubstitutor,
    pub substitution_rollbacks: SortedMapTable<TextRange, UString>,
    pub shaping_decisions: Vec<ShapingDecisionInfo>,
    pub metric_decisions: Vec<ClusterMetricDecision>,
    pub punctuation_atoms: Vec<PunctuationAtom>,
    pub geometry_decisions: Vec<ClusterGeometryDecisionInfo>,
    pub spacing_plan: PunctuationSpacingCompressionResult,
    pub attached_punctuation_boundary: AttachedInlinePunctuationBoundaryResult,
    pub role_override_infos: Vec<RoleOverrideInfo>,
    pub laid_out_lines: Vec<LineBox>,
    pub line_solution: LineSolution,
    pub clusters: Vec<Cluster>,
    pub justification_plans: Vec<Option<JustificationPlan>>,
    pub auto_space_decisions: Vec<AutoSpaceDecisionInfo>,
    pub edge_trim_decisions: Vec<LineEdgeTrimDecisionInfo>,
    pub decoration_decisions: Vec<DecorationDecisionInfo>,
    pub decoration_segments: Vec<DecorationSegmentInfo>,
    pub ruby_decisions: Vec<RubyDecisionInfo>,
    pub bopomofo_decisions: Vec<BopomofoDecisionInfo>,
    pub mandatory_break_decisions: Vec<MandatoryBreakDecisionInfo>,
    pub max_lines_decision: Option<MaxLinesDecisionInfo>,
    pub line_spacing_decision: Option<LineSpacingDecisionInfo>,
    pub ruby_line_height_decision: Option<RubyLineHeightDecisionInfo>,
    pub inline_object_line_height_decision: Option<InlineObjectLineHeightDecisionInfo>,
    pub kinsoku_decision: KinsokuDecisionInfo,
    pub contextual_kinsoku_decisions: Vec<ContextualKinsokuDecisionInfo>,
    pub line_length_grid_decision: LineLengthGridDecisionInfo,
    pub first_line_indent_decision: FirstLineIndentDecisionInfo,
    pub inline_box_decisions: Vec<InlineBoxDecisionInfo>,
    pub inline_object_decisions: Vec<InlineObjectDecisionInfo>,
    pub inline_object_punctuation_attachment_decisions: Vec<InlineObjectPunctuationAttachmentDecisionInfo>,
    pub zero_width_break_decisions: Vec<ZeroWidthBreakDecisionInfo>,
    pub break_opportunity_decisions: Vec<BreakOpportunityDecisionInfo>,
    pub emergency_tracking_eligibility_decisions: Vec<EmergencyTrackingEligibilityDecisionInfo>,
    pub progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>,
}

impl LayoutDebugStageInput {
    pub fn new(text: &UStr, font_decisions: Vec<FontDecision>, punctuation_glyph_substitutor: ClreqPunctuationGlyphSubstitutor, substitution_rollbacks: SortedMapTable<TextRange, UString>, shaping_decisions: Vec<ShapingDecisionInfo>, metric_decisions: Vec<ClusterMetricDecision>, punctuation_atoms: Vec<PunctuationAtom>, geometry_decisions: Vec<ClusterGeometryDecisionInfo>, spacing_plan: PunctuationSpacingCompressionResult, attached_punctuation_boundary: AttachedInlinePunctuationBoundaryResult, role_override_infos: Vec<RoleOverrideInfo>, laid_out_lines: Vec<LineBox>, line_solution: LineSolution, clusters: Vec<Cluster>, justification_plans: Vec<Option<JustificationPlan>>, auto_space_decisions: Vec<AutoSpaceDecisionInfo>, edge_trim_decisions: Vec<LineEdgeTrimDecisionInfo>, decoration_decisions: Vec<DecorationDecisionInfo>, decoration_segments: Vec<DecorationSegmentInfo>, ruby_decisions: Vec<RubyDecisionInfo>, bopomofo_decisions: Vec<BopomofoDecisionInfo>, mandatory_break_decisions: Vec<MandatoryBreakDecisionInfo>, max_lines_decision: Option<MaxLinesDecisionInfo>, line_spacing_decision: Option<LineSpacingDecisionInfo>, ruby_line_height_decision: Option<RubyLineHeightDecisionInfo>, inline_object_line_height_decision: Option<InlineObjectLineHeightDecisionInfo>, kinsoku_decision: KinsokuDecisionInfo, contextual_kinsoku_decisions: Vec<ContextualKinsokuDecisionInfo>, line_length_grid_decision: LineLengthGridDecisionInfo, first_line_indent_decision: FirstLineIndentDecisionInfo, inline_box_decisions: Vec<InlineBoxDecisionInfo>, inline_object_decisions: Vec<InlineObjectDecisionInfo>, inline_object_punctuation_attachment_decisions: Vec<InlineObjectPunctuationAttachmentDecisionInfo>, zero_width_break_decisions: Vec<ZeroWidthBreakDecisionInfo>, break_opportunity_decisions: Vec<BreakOpportunityDecisionInfo>, emergency_tracking_eligibility_decisions: Vec<EmergencyTrackingEligibilityDecisionInfo>, progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>) -> Self {
        Self {
            text: text.to_ustring(),
            font_decisions,
            punctuation_glyph_substitutor,
            substitution_rollbacks,
            shaping_decisions,
            metric_decisions,
            punctuation_atoms,
            geometry_decisions,
            spacing_plan,
            attached_punctuation_boundary,
            role_override_infos,
            laid_out_lines,
            line_solution,
            clusters,
            justification_plans,
            auto_space_decisions,
            edge_trim_decisions,
            decoration_decisions,
            decoration_segments,
            ruby_decisions,
            bopomofo_decisions,
            mandatory_break_decisions,
            max_lines_decision,
            line_spacing_decision,
            ruby_line_height_decision,
            inline_object_line_height_decision,
            kinsoku_decision,
            contextual_kinsoku_decisions,
            line_length_grid_decision,
            first_line_indent_decision,
            inline_box_decisions,
            inline_object_decisions,
            inline_object_punctuation_attachment_decisions,
            zero_width_break_decisions,
            break_opportunity_decisions,
            emergency_tracking_eligibility_decisions,
            progressive_break_opportunities,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LayoutDebugStageInput(")); __s += &(UString::from("text=")); __s += (self.text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontDecisions=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationGlyphSubstitutor=")); __s += UString::from(format!("{}", (self.punctuation_glyph_substitutor).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("substitutionRollbacks=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.substitution_rollbacks).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", (self.substitution_rollbacks).clone().key_at(i).to_string(), (self.substitution_rollbacks).clone().value_at(i));
            i += 1;
        }
        out.push('}');
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationAtoms=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.punctuation_atoms).clone();
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("spacingPlan=")); __s += UString::from(format!("{}", (self.spacing_plan).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("attachedPunctuationBoundary=")); __s += UString::from(format!("{}", (self.attached_punctuation_boundary).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("roleOverrideInfos=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.role_override_infos).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("laidOutLines=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.laid_out_lines).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineSolution=")); __s += UString::from(format!("{}", (self.line_solution).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("clusters=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.clusters).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("justificationPlans=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.justification_plans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", match arr[i] { Some(ref v) => UString::from(format!("{}", v.to_string()).as_str()), None => UString::from("null") });
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("edgeTrimDecisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.edge_trim_decisions).clone();
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("maxLinesDecision=")); __s += (match &(self.max_lines_decision) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineSpacingDecision=")); __s += (match &(self.line_spacing_decision) { None => UString::from("null"), Some(__option1) => UString::from(format!("{}", __option1.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rubyLineHeightDecision=")); __s += (match &(self.ruby_line_height_decision) { None => UString::from("null"), Some(__option2) => UString::from(format!("{}", __option2.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineObjectLineHeightDecision=")); __s += (match &(self.inline_object_line_height_decision) { None => UString::from("null"), Some(__option3) => UString::from(format!("{}", __option3.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("kinsokuDecision=")); __s += UString::from(format!("{}", (self.kinsoku_decision).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("contextualKinsokuDecisions=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineLengthGridDecision=")); __s += UString::from(format!("{}", (self.line_length_grid_decision).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("firstLineIndentDecision=")); __s += UString::from(format!("{}", (self.first_line_indent_decision).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineBoxDecisions=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("progressiveBreakOpportunities=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.progressive_break_opportunities).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.progressive_break_opportunities).clone().key_at(i)) as i32).to_ne_bytes()), (self.progressive_break_opportunities).clone().value_at(i).to_string());
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, Copy)]
pub struct LayoutDebugAssembly;

impl LayoutDebugAssembly {
    pub(crate) fn layout_debug_assembly_repair_candidate_to_decision_info(candidate: RepairCandidate, clusters: &Vec<Cluster>) -> LineRepairCandidateInfo {
        return LineRepairCandidateInfo::new((candidate.kind).to_ustring().as_ustr(), (candidate.reason_code).to_ustring().as_ustr(), ((clusters[usize::try_from(candidate.offender_cluster_index).unwrap_or(0)]).clone().range).clone(), candidate.penalty, candidate.accepted, candidate.rejection_reason.clone(), candidate.target_cluster_index, candidate.carried_cluster_index, Some(candidate.shrink), Some(candidate.required_shrink), Some(candidate.available_capacity));
    }

    pub(crate) fn layout_debug_assembly_to_push_in_allocations(allocations: &Vec<PushInAllocation>, clusters: &Vec<Cluster>) -> Vec<LineRepairAllocationInfo> {
        let capacity = allocations.len();
        let mut push_in_allocations = Vec::with_capacity(capacity);
        for a_idx in 0..match u32::try_from(allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let alloc = (allocations[usize::try_from(a_idx).unwrap_or(0)]).clone();
            push_in_allocations.push(LineRepairAllocationInfo::new(((clusters[usize::try_from(alloc.cluster_index).unwrap_or(0)]).clone().range).clone(), alloc.shrink, alloc.available_capacity));
        }
        return push_in_allocations;
    }

    pub(crate) fn layout_debug_assembly_repair_option_to_decision_info(repair: RepairOption, clusters: &Vec<Cluster>) -> LineRepairDecisionInfo {
        return match repair {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, total_available_capacity: _p5 } => {
            let push_in_allocations = LayoutDebugAssembly::layout_debug_assembly_to_push_in_allocations(&_p3, &clusters);
            let colon_idx = u_string::find_from(&(_p1), UString::from(":").as_ustr(), 0);
            let reason_code = if colon_idx >= 0 { u_string::substring(&_p1, 0i32, i32::from_ne_bytes(((colon_idx) as i32).to_ne_bytes())).to_ustring() } else { _p1.to_ustring() };
            LineRepairDecisionInfo::new(&(UStr::new(&[80,117,115,104,73,110])), reason_code.as_ustr(), ((clusters[usize::try_from(_p2).unwrap_or(0)]).clone().range).clone(), _p0, Some(_p2), None, Some(_p4), Some(_p5), Some((push_in_allocations).clone()))
        },
            RepairOption::Hang { penalty: _p0, reason: _p1,
offender_cluster_index: _p2 } => LineRepairDecisionInfo::new(&(UStr::new(&[72,97,110,103])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), ((clusters[usize::try_from(_p2).unwrap_or(0)]).clone().range).clone(), _p0, None, None, Some(0.0), Some(0.0), Some(vec![])),
            RepairOption::CarryPrevious { penalty: _p0, reason: _p1, offender_cluster_index: _p2,
carried_cluster_index: _p3 } => LineRepairDecisionInfo::new(&(UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), ((clusters[usize::try_from(_p2).unwrap_or(0)]).clone().range).clone(), _p0, None, Some(_p3), Some(0.0), Some(0.0), Some(vec![])),
            RepairOption::CarryNext { penalty: _p0, reason: _p1,
moved_cluster_index: _p2 } => LineRepairDecisionInfo::new(&(UStr::new(&[67,97,114,114,121,78,101,120,116])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,69,110,100])), ((clusters[usize::try_from(_p2).unwrap_or(0)]).clone().range).clone(), _p0, None, Some(_p2), Some(0.0), Some(0.0), Some(vec![])),
            RepairOption::LeaveRagged { penalty: _p0, reason: _p1,
offender_cluster_index: _p2 } => LineRepairDecisionInfo::new(&(UStr::new(&[76,101,97,118,101,82,97,103,103,101,100])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), ((clusters[usize::try_from(_p2).unwrap_or(0)]).clone().range).clone(), _p0, None, None, Some(0.0), Some(0.0), Some(vec![])),
        };
    }

    pub(crate) fn layout_debug_assembly_repair_option_name(repair: RepairOption) -> UString {
        return match repair {
            RepairOption::PushIn { .. } => UString::from("PushIn").to_ustring(),
            RepairOption::Hang { .. } => UString::from("Hang").to_ustring(),
            RepairOption::CarryPrevious { .. } => UString::from("CarryPrevious").to_ustring(),
            RepairOption::CarryNext { .. } => UString::from("CarryNext").to_ustring(),
            RepairOption::LeaveRagged { .. } => UString::from("LeaveRagged").to_ustring(),
        };
    }

    pub(crate) fn layout_debug_assembly_progressive_break_tier_name(tier: ProgressiveBreakTier) -> UString {
        if tier == ProgressiveBreakTier::Whitespace {
            return UString::from("Whitespace").to_ustring();
        }
        if tier == ProgressiveBreakTier::Structural {
            return UString::from("Structural").to_ustring();
        }
        if tier == ProgressiveBreakTier::Syllable {
            return UString::from("Syllable").to_ustring();
        }
        if tier == ProgressiveBreakTier::WholeToken {
            return UString::from("WholeToken").to_ustring();
        }
        if tier == ProgressiveBreakTier::Emergency {
            return UString::from("Emergency").to_ustring();
        }
        return UString::from("Unknown").to_ustring();
    }

    pub fn layout_debug_assembly_build_layout_debug_info(engine: ExplainableStubParagraphLayoutEngine, stage: LayoutDebugStageInput) -> Result<LayoutDebugInfo, UStringFault> {
        let capacity = stage.font_decisions.len();
        let mut font_decisions_out = Vec::with_capacity(capacity);
        for f_idx in 0..match u32::try_from(stage.font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let decision = (stage.font_decisions[usize::try_from(f_idx).unwrap_or(0)]).clone();
            let cluster_text = u_string::substring(&(stage.text).to_ustring(), i32::from_ne_bytes((((decision.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((decision.range).clone().end) as i32).to_ne_bytes()));
            let substitution = ContextualPunctuationDisplaySubstitutionFns::contextual_punctuation_display_substitution_fns_substitute_for_role((stage.punctuation_glyph_substitutor).clone(), cluster_text.as_ustr(), decision.role)?;
            let mut rollback_cause: Option<UString> = None;
            for i in 0..u32::from_ne_bytes(((stage.substitution_rollbacks.size()) as u32).to_ne_bytes()) {
                let k = stage.substitution_rollbacks.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
                if i32::from_ne_bytes(((k.start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((decision.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((k.end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((decision.range).clone().end) as i32).to_ne_bytes()) {
                    rollback_cause = Some(stage.substitution_rollbacks.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }).clone());
                    break;
                }
            }
            let display_text = if rollback_cause.is_some() { (cluster_text.to_ustring()).clone() } else { (substitution.display_text).to_ustring() };
            let substitution_reason = match &(rollback_cause) { Some(__option5) => UString::from(format!("{}", { let mut __s = UString::new(); __s += (substitution.reason).to_ustring().as_ustr(); __s += &(UString::from(":")); __s += __option5.as_ustr(); __s }).as_str()),
None => ((substitution.reason).to_ustring()).clone() };
            font_decisions_out.push(FontDecisionInfo::new((decision.range).clone(), cluster_text.as_ustr(), display_text.as_ustr(), UString::from(decision.role.name()).as_ustr(), ((decision.candidate).clone().key).to_ustring().as_ustr(), (decision.reason).to_ustring().as_ustr(), substitution_reason.as_ustr()));
        }
        let capacity = stage.metric_decisions.len();
        let mut metric_decisions_out = Vec::with_capacity(capacity);
        for m_idx in 0..match u32::try_from(stage.metric_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let decision = (stage.metric_decisions[usize::try_from(m_idx).unwrap_or(0)]).clone();
            metric_decisions_out.push(MetricDecisionInfo::new((decision.range).clone(), (decision.source_text).to_ustring().as_ustr(), UString::from((decision.request).clone().role.name()).as_ustr(), ((decision.request).clone().font_key).to_ustring().as_ustr(), (decision.raw_metrics).clone().ascent, (decision.raw_metrics).clone().descent, (decision.raw_metrics).clone().leading, UString::from((decision.raw_metrics).clone().source.name()).as_ustr(), (decision.layout_metrics).clone().ascent, (decision.layout_metrics).clone().descent, UString::from((decision.layout_metrics).clone().baseline_class.name()).as_ustr(), UString::from((decision.layout_metrics).clone().metric_box.name()).as_ustr(), UString::from((decision.layout_metrics).clone().source.name()).as_ustr(), ((decision.layout_metrics).clone().reason).to_ustring().as_ustr()));
        }
        let capacity = stage.punctuation_atoms.len();
        let mut punctuation_decisions_out = Vec::with_capacity(capacity);
        for p_idx in 0..match u32::try_from(stage.punctuation_atoms.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let atom = (stage.punctuation_atoms[usize::try_from(p_idx).unwrap_or(0)]).clone();
            punctuation_decisions_out.push(PunctuationDecisionInfo::new((atom.range).clone(), (atom.char).to_ustring().as_ustr(), UString::from(atom.punctuation_class.name()).as_ustr(), atom.advance, atom.body_width, (atom.leading_glue).clone().natural, (atom.trailing_glue).clone().natural, UString::from(atom.anchor.name()).as_ustr(), (atom.ink_bounds).clone(), Some((atom.geometry_source).to_ustring()), atom.policy_body_floor, atom.ink_width, atom.ink_center, atom.ink_containment_body_floor, Some(atom.ink_containment_applied), atom.ink_bounds_fallback.clone(), atom.halt_advance, atom.halt_validation.clone(), Some(atom.advance_expansion), Some(atom.glyph_inline_shift), atom.glyph_placement_reason.clone(), Some(atom.leading_glue_initially_consumed), Some(atom.trailing_glue_initially_consumed)));
        }
        let capacity = (stage.spacing_plan).clone().adjustments.len();
        let mut spacing_decisions_out = Vec::with_capacity(capacity);
        for s_idx in 0..match u32::try_from((stage.spacing_plan).clone().adjustments.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let adjustment = ((stage.spacing_plan).clone().adjustments[usize::try_from(s_idx).unwrap_or(0)]).clone();
            spacing_decisions_out.push(SpacingDecisionInfo::new((adjustment.range).clone(), (adjustment.left_char).to_ustring().as_ustr(), (adjustment.right_char).to_ustring().as_ustr(), adjustment.natural_inner_glue, adjustment.adjusted_inner_glue, adjustment.reduction, (adjustment.reduction_target_range).clone(), (adjustment.reason).to_ustring().as_ustr()));
        }
        for apb_idx in 0..match u32::try_from((stage.attached_punctuation_boundary).clone().decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            spacing_decisions_out.push(((stage.attached_punctuation_boundary).clone().decisions[usize::try_from(apb_idx).unwrap_or(0)]).clone());
        }
        let mut line_decisions_out: Vec<LineDecisionInfo> = Vec::new();
        let min_line_count = if i32::from_ne_bytes(((u32::try_from((stage.laid_out_lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::try_from(((stage.line_solution).clone().lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { u32::try_from((stage.laid_out_lines.len()) & 0xFFFF_FFFF).unwrap_or(0) } else { u32::try_from(((stage.line_solution).clone().lines.len()) & 0xFFFF_FFFF).unwrap_or(0) };
        for line_index in 0..min_line_count {
            let line = (stage.laid_out_lines[usize::try_from(line_index).unwrap_or(0)]).clone();
            let candidate = ((stage.line_solution).clone().lines[usize::try_from(line_index).unwrap_or(0)]).clone();
            let repair_name = match &(candidate.repair) { Some(__option6) => Some(LayoutDebugAssembly::layout_debug_assembly_repair_option_name((*__option6).clone()).to_ustring()), None => None };
            let repair_penalty = match &(candidate.repair) { Some(__option7) => RepairOptions::repair_options_penalty((*__option7).clone()), None => 0 };
            let repair_decision = match &(candidate.repair) { Some(__option8) => Some(LayoutDebugAssembly::layout_debug_assembly_repair_option_to_decision_info((*__option8).clone(), &stage.clusters)), None => None };
            let capacity = candidate.repair_candidates.len();
            let mut repair_candidates_out = Vec::with_capacity(capacity);
            for rc_idx in 0..match u32::try_from(candidate.repair_candidates.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                repair_candidates_out.push(LayoutDebugAssembly::layout_debug_assembly_repair_candidate_to_decision_info((candidate.repair_candidates[usize::try_from(rc_idx).unwrap_or(0)]).clone(), &stage.clusters));
            }
            let mut notes = vec![
    { let mut __s = UString::new(); __s += &(UString::from("index:")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(line_index)).as_str())); __s }.clone(),
    { let mut __s = UString::new(); __s += &(UString::from("end:")); __s += UString::from(line.end_reason.name()).as_ustr(); __s }.clone(),
    { let mut __s = UString::new(); __s += &(UString::from("natural:")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(line.natural_width)); __s }.clone(),
    { let mut __s = UString::new(); __s += &(UString::from("adjusted:")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(line.adjusted_width)); __s }.clone(),
    { let mut __s = UString::new(); __s += &(UString::from("visual:")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(line.visual_width)); __s }.clone(),
];
            let next_cluster = u32::wrapping_add(candidate.cluster_range.end, 1);
            if stage.progressive_break_opportunities.has(&(next_cluster)) {
                let opp = (stage.progressive_break_opportunities.get(&(next_cluster))).unwrap();
                notes.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("technical-break:")); __s += LayoutDebugAssembly::layout_debug_assembly_progressive_break_tier_name(opp.tier).as_ustr(); __s }).as_str()));
            }
            match &(candidate.repair) {
                Some(__option9) => {
                    notes.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("repair-reason:")); __s += RepairOptions::repair_options_reason((*__option9).clone()).as_ustr(); __s }).as_str()));
                }
                None => {
                }
            }
            let plan = if ({ let v: u32 = line_index; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u32::try_from((stage.justification_plans.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { (stage.justification_plans[usize::try_from(line_index).unwrap_or(0)]).clone() } else { None };
            match &(plan) {
                Some(__option10) => {
                    if __option10.fallback_reason.is_some() {
                    notes.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("justify-fallback:")); __s += match &(__option10.fallback_reason) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s }).as_str()));
                    }
                }
                None => {
                }
            }
            line_decisions_out.push(LineDecisionInfo::new((line.range).clone(), engine.line_breaker.get_strategy_name().as_ustr(), repair_name.clone(), Some(repair_penalty), (repair_decision).clone(), Some((repair_candidates_out).clone()), Some((notes).clone())));
        }
        let mut justification_decisions_out: Vec<JustificationDecisionInfo> = Vec::new();
        for i in 0..match u32::try_from((stage.line_solution).clone().lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let candidate = ((stage.line_solution).clone().lines[usize::try_from(i).unwrap_or(0)]).clone();
            let plan = if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u32::try_from((stage.justification_plans.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { (stage.justification_plans[usize::try_from(i).unwrap_or(0)]).clone() } else { None };
            match &(plan) {
                Some(__option11) => {
                    if i32::from_ne_bytes(((u32::try_from((__option11.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) || (__option11.deficit_before) > (0.0f64) {
                    let capacity = __option11.allocations.len();
                    let mut allocations_out = Vec::with_capacity(capacity);
                    for a_idx in 0..match u32::try_from(__option11.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let alloc = (__option11.allocations[usize::try_from(a_idx).unwrap_or(0)]).clone();
                        allocations_out.push(JustificationAllocationInfo::new(((stage.clusters[usize::try_from(alloc.target_cluster_index).unwrap_or(0)]).clone().range).clone(), UString::from(alloc.kind.name()).as_ustr(), alloc.priority, alloc.delta, (alloc.reason).to_ustring().as_ustr()));
                    }
                    justification_decisions_out.push(JustificationDecisionInfo::new((candidate.source_range).clone(), __option11.deficit_before, __option11.unfilled_deficit, allocations_out.to_vec()));
                    }
                }
                None => {
                }
            }
        }
        return Ok(LayoutDebugInfo::new((stage.max_lines_decision).clone(), Some((metric_decisions_out).clone()), Some((stage.geometry_decisions).clone()), Some((stage.auto_space_decisions).clone()), Some((stage.ruby_decisions).clone()), Some((stage.bopomofo_decisions).clone()), Some((font_decisions_out).clone()), Some((stage.shaping_decisions).clone()), Some((punctuation_decisions_out).clone()), Some((spacing_decisions_out).clone()), Some((stage.role_override_infos).clone()), Some((line_decisions_out).clone()), Some((justification_decisions_out).clone()), Some((stage.edge_trim_decisions).clone()), Some((stage.decoration_decisions).clone()), Some((stage.decoration_segments).clone()), Some((stage.mandatory_break_decisions).clone()), (stage.line_spacing_decision).clone(), (stage.ruby_line_height_decision).clone(), (stage.inline_object_line_height_decision).clone(), Some((stage.kinsoku_decision).clone()), Some((stage.contextual_kinsoku_decisions).clone()), Some((stage.line_length_grid_decision).clone()), Some((stage.first_line_indent_decision).clone()), Some((stage.inline_box_decisions).clone()), Some((stage.inline_object_decisions).clone()), Some((stage.inline_object_punctuation_attachment_decisions).clone()), Some((stage.zero_width_break_decisions).clone()), Some((stage.break_opportunity_decisions).clone()), Some((stage.emergency_tracking_eligibility_decisions).clone())));
    }
}
