use crate::org::tiqian::clreq::adjustment_style_policy::AdjustmentStylePolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::clreq::number_symbol_cohesion::NumberSymbolCohesion;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::resolved_kinsoku::ResolvedKinsoku;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::break_opportunity_decision_info::BreakOpportunityDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::emergency_tracking_eligibility_decision_info::EmergencyTrackingEligibilityDecisionInfo;
use crate::org::tiqian::core::first_line_indent_decision_info::FirstLineIndentDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_punctuation_attachment_decision_info::InlineObjectPunctuationAttachmentDecisionInfo;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::kinsoku_decision_info::KinsokuDecisionInfo;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_length_grid_decision_info::LineLengthGridDecisionInfo;
use crate::org::tiqian::core::line_spacing_decision_info::LineSpacingDecisionInfo;
use crate::org::tiqian::core::mandatory_break_decision_info::MandatoryBreakDecisionInfo;
use crate::org::tiqian::core::role_override_info::RoleOverrideInfo;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::zero_width_break_decision_info::ZeroWidthBreakDecisionInfo;
use crate::org::tiqian::font::font_metrics::FontMetricsNormalizationInput;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::layout_font_metrics::LayoutFontMetrics;
use crate::org::tiqian::font::metric_box::MetricBox;
use crate::org::tiqian::layout::annotation_geometry_stage::RubyFontGeometry;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_geometry_stage::ClusterMetricDecision;
use crate::org::tiqian::layout::line_geometry_stage::LineGeometryStageFns;
use crate::org::tiqian::layout::line_geometry_stage::ResolvedLineMetrics;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStage;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::layout::punctuation_geometry_ledger::AttachedInlinePunctuationBoundaryResult;
use crate::org::tiqian::layout::punctuation_geometry_ledger::PunctuationGeometryLedger;
use crate::org::tiqian::layout::punctuation_geometry_stage::ContextualKinsoku;
use crate::org::tiqian::layout::punctuation_geometry_stage::InlineBoxApplicationResult;
use crate::org::tiqian::layout::punctuation_geometry_stage::InlineObjectAttachedMark;
use crate::org::tiqian::layout::punctuation_geometry_stage::PunctuationGeometryStage;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressionResult;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaries;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaryResolver;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Clone)]
pub struct ParagraphLayoutPrep {
    pub input: LayoutInput,
    pub rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>,
    pub text: String,
    pub font_size: f64,
    pub style_at: Arc<dyn Fn(u32) -> TextStyle + Send + Sync>,
    pub font_size_at: Arc<dyn Fn(u32) -> f64 + Send + Sync>,
    pub bopomofo_font_weight_at: Arc<dyn Fn(u32) -> u32 + Send + Sync>,
    pub ruby_font_size: f64,
    pub ruby_stack_gap: f64,
    pub ruby_font_weight: u32,
    pub pinyin_spans: Vec<RubySpan>,
    pub clreq_profile: ClreqProfile,
    pub punctuation_glyph_substitutor: ClreqPunctuationGlyphSubstitutor,
    pub measure: f64,
    pub measure_em: f64,
    pub grid_body_offset: f64,
    pub line_length_grid_decision: LineLengthGridDecisionInfo,
    pub quote_pairs: Vec<QuotePair>,
    pub role_override_infos: Vec<RoleOverrideInfo>,
    pub font_decisions: Vec<FontDecision>,
    pub hyphen_offsets: SortedSetTable<u32>,
    pub hyphen_advance: f64,
    pub hyphen_glyphs: Vec<Glyph>,
    pub substitution_rollbacks: SortedMapTable<TextRange, String>,
    pub break_opportunity_decisions: Vec<BreakOpportunityDecisionInfo>,
    pub emergency_tracking_eligibility_decisions: Vec<EmergencyTrackingEligibilityDecisionInfo>,
    pub progressive_break_offsets: SortedMapTable<u32, ProgressiveBreakOpportunity>,
    pub shaped_glyphs_by_cluster_range: SortedMapTable<TextRange, Vec<Glyph>>,
    pub open_type_features_by_cluster_range: SortedMapTable<TextRange, Vec<String>>,
    pub shaping_decisions: Vec<ShapingDecisionInfo>,
    pub east_asian_spacing_edges: Vec<EastAsianSpacingEdges>,
    pub auto_space_decisions: Vec<AutoSpaceDecisionInfo>,
    pub inline_box_result: InlineBoxApplicationResult,
    pub natural_clusters: Vec<Cluster>,
    pub inline_object_by_cluster_index: SortedMapTable<u32, InlineObjectSpan>,
    pub uniform_inline_object_boundary_after_clusters: SortedSetTable<u32>,
    pub preferred_inline_object_boundary_after_clusters: SortedMapTable<u32, InlineObjectPreferredStretch>,
    pub inline_object_boundary_unbreakable_ranges: Vec<IntRange>,
    pub cluster_roles: Vec<FontRole>,
    pub resolved_kinsoku: ResolvedKinsoku,
    pub kinsoku_rule: ClreqKinsokuRule,
    pub inline_object_attached_marks: Vec<InlineObjectAttachedMark>,
    pub inline_object_separator_space_trims: SortedMapTable<u32, f64>,
    pub inline_object_attachment_no_stretch_boundaries: SortedSetTable<u32>,
    pub inline_object_punctuation_attachment_decisions: Vec<InlineObjectPunctuationAttachmentDecisionInfo>,
    pub mandatory_break_clusters: SortedSetTable<u32>,
    pub zero_width_break_clusters: SortedSetTable<u32>,
    pub mandatory_break_decisions: Vec<MandatoryBreakDecisionInfo>,
    pub zero_width_break_decisions: Vec<ZeroWidthBreakDecisionInfo>,
    pub punctuation_atoms: Vec<PunctuationAtom>,
    pub spacing_plan: PunctuationSpacingCompressionResult,
    pub ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>,
    pub ruby_and_bopomofo_spread: SortedMapTable<u32, f64>,
    pub natural_inline_attachments: Vec<InlineAttachment>,
    pub attached_punctuation_boundary: AttachedInlinePunctuationBoundaryResult,
    pub base_geometry: PunctuationGeometryLedger,
    pub attached_punctuation_trailing_glue_by_cluster: SortedMapTable<u32, f64>,
    pub clusters: Vec<Cluster>,
    pub adjustment_style: AdjustmentStylePolicy,
    pub atom_class_by_range: SortedMapTable<TextRange, PunctuationClass>,
    pub shrink_opportunities: Vec<ShrinkOpportunity>,
}

impl ParagraphLayoutPrep {
    pub fn new(input: LayoutInput, rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>, text: &str, font_size: f64, style_at: Arc<dyn Fn(u32) -> TextStyle + Send + Sync>, font_size_at: Arc<dyn Fn(u32) -> f64 + Send + Sync>, bopomofo_font_weight_at:
Arc<dyn Fn(u32) -> u32 + Send + Sync>, ruby_font_size: f64, ruby_stack_gap: f64, ruby_font_weight: u32, pinyin_spans: Vec<RubySpan>, clreq_profile: ClreqProfile, punctuation_glyph_substitutor: ClreqPunctuationGlyphSubstitutor, measure: f64, measure_em: f64, grid_body_offset: f64,
line_length_grid_decision: LineLengthGridDecisionInfo, quote_pairs: Vec<QuotePair>, role_override_infos: Vec<RoleOverrideInfo>, font_decisions: Vec<FontDecision>, hyphen_offsets: SortedSetTable<u32>, hyphen_advance: f64, hyphen_glyphs: Vec<Glyph>, substitution_rollbacks:
SortedMapTable<TextRange, String>, break_opportunity_decisions: Vec<BreakOpportunityDecisionInfo>, emergency_tracking_eligibility_decisions: Vec<EmergencyTrackingEligibilityDecisionInfo>, progressive_break_offsets: SortedMapTable<u32, ProgressiveBreakOpportunity>,
shaped_glyphs_by_cluster_range: SortedMapTable<TextRange, Vec<Glyph>>, open_type_features_by_cluster_range: SortedMapTable<TextRange, Vec<String>>, shaping_decisions: Vec<ShapingDecisionInfo>, east_asian_spacing_edges: Vec<EastAsianSpacingEdges>, auto_space_decisions:
Vec<AutoSpaceDecisionInfo>, inline_box_result: InlineBoxApplicationResult, natural_clusters: Vec<Cluster>, inline_object_by_cluster_index: SortedMapTable<u32, InlineObjectSpan>, uniform_inline_object_boundary_after_clusters: SortedSetTable<u32>,
preferred_inline_object_boundary_after_clusters: SortedMapTable<u32, InlineObjectPreferredStretch>, inline_object_boundary_unbreakable_ranges: Vec<IntRange>, cluster_roles: Vec<FontRole>, resolved_kinsoku: ResolvedKinsoku, kinsoku_rule: ClreqKinsokuRule,
inline_object_attached_marks: Vec<InlineObjectAttachedMark>, inline_object_separator_space_trims: SortedMapTable<u32, f64>, inline_object_attachment_no_stretch_boundaries: SortedSetTable<u32>, inline_object_punctuation_attachment_decisions:
Vec<InlineObjectPunctuationAttachmentDecisionInfo>, mandatory_break_clusters: SortedSetTable<u32>, zero_width_break_clusters: SortedSetTable<u32>, mandatory_break_decisions: Vec<MandatoryBreakDecisionInfo>, zero_width_break_decisions: Vec<ZeroWidthBreakDecisionInfo>,
punctuation_atoms: Vec<PunctuationAtom>, spacing_plan: PunctuationSpacingCompressionResult, ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>, ruby_and_bopomofo_spread: SortedMapTable<u32, f64>, natural_inline_attachments: Vec<InlineAttachment>,
attached_punctuation_boundary: AttachedInlinePunctuationBoundaryResult, base_geometry: PunctuationGeometryLedger, attached_punctuation_trailing_glue_by_cluster: SortedMapTable<u32, f64>, clusters: Vec<Cluster>, adjustment_style: AdjustmentStylePolicy, atom_class_by_range:
SortedMapTable<TextRange, PunctuationClass>, shrink_opportunities: Vec<ShrinkOpportunity>) -> Self {
        Self {
            input,
            rejected_technical_tiers_by_span,
            text: text.to_string(),
            font_size,
            style_at,
            font_size_at,
            bopomofo_font_weight_at,
            ruby_font_size,
            ruby_stack_gap,
            ruby_font_weight,
            pinyin_spans,
            clreq_profile,
            punctuation_glyph_substitutor,
            measure,
            measure_em,
            grid_body_offset,
            line_length_grid_decision,
            quote_pairs,
            role_override_infos,
            font_decisions,
            hyphen_offsets,
            hyphen_advance,
            hyphen_glyphs,
            substitution_rollbacks,
            break_opportunity_decisions,
            emergency_tracking_eligibility_decisions,
            progressive_break_offsets,
            shaped_glyphs_by_cluster_range,
            open_type_features_by_cluster_range,
            shaping_decisions,
            east_asian_spacing_edges,
            auto_space_decisions,
            inline_box_result,
            natural_clusters,
            inline_object_by_cluster_index,
            uniform_inline_object_boundary_after_clusters,
            preferred_inline_object_boundary_after_clusters,
            inline_object_boundary_unbreakable_ranges,
            cluster_roles,
            resolved_kinsoku,
            kinsoku_rule,
            inline_object_attached_marks,
            inline_object_separator_space_trims,
            inline_object_attachment_no_stretch_boundaries,
            inline_object_punctuation_attachment_decisions,
            mandatory_break_clusters,
            zero_width_break_clusters,
            mandatory_break_decisions,
            zero_width_break_decisions,
            punctuation_atoms,
            spacing_plan,
            ruby_font_geometry_by_span,
            ruby_and_bopomofo_spread,
            natural_inline_attachments,
            attached_punctuation_boundary,
            base_geometry,
            attached_punctuation_trailing_glue_by_cluster,
            clusters,
            adjustment_style,
            atom_class_by_range,
            shrink_opportunities,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LineBreakPlanningStageResult {
    pub metric_decisions: Vec<ClusterMetricDecision>,
    pub metric_decision_by_range: SortedMapTable<TextRange, ClusterMetricDecision>,
    pub base_ascent: f64,
    pub base_descent: f64,
    pub base_box_descent: f64,
    pub base_face_height: f64,
    pub existing_interline_space: f64,
    pub ruby_extent: f64,
    pub base_line_metrics: ResolvedLineMetrics,
    pub line_spacing_decision: Option<LineSpacingDecisionInfo>,
    pub block_indent: f64,
    pub first_line_indent: f64,
    pub first_line_indent_decision: FirstLineIndentDecisionInfo,
    pub kinsoku_decision: KinsokuDecisionInfo,
    pub ascii_point_mark_kinsoku: ContextualKinsoku,
    pub inline_object_kinsoku: ContextualKinsoku,
    pub unicode_punctuation_boundaries: UnicodePunctuationBoundaries,
    pub western_bracket_cjk_inter_char_boundary_after_clusters: SortedSetTable<u32>,
    pub attached_inline_physical_boundary_after_clusters: SortedSetTable<u32>,
    pub attached_inline_virtual_boundary_after_clusters: SortedMapTable<u32, u32>,
    pub attached_inline_virtual_sino_western_boundary_after_clusters: SortedSetTable<u32>,
    pub no_stretch_boundary_clusters: SortedSetTable<u32>,
    pub no_stretch_boundary_after_clusters: SortedSetTable<u32>,
    pub technical_boundary_after_clusters: SortedMapTable<u32, ProgressiveBreakTier>,
    pub emergency_tracking_boundary_after_clusters: SortedMapTable<u32, String>,
    pub progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>,
    pub line_solution: LineSolution,
}

impl LineBreakPlanningStageResult {
    pub fn new(metric_decisions: Vec<ClusterMetricDecision>, metric_decision_by_range: SortedMapTable<TextRange, ClusterMetricDecision>, base_ascent: f64, base_descent: f64, base_box_descent: f64, base_face_height: f64, existing_interline_space: f64, ruby_extent: f64,
base_line_metrics: ResolvedLineMetrics, line_spacing_decision: Option<LineSpacingDecisionInfo>, block_indent: f64, first_line_indent: f64, first_line_indent_decision: FirstLineIndentDecisionInfo, kinsoku_decision: KinsokuDecisionInfo, ascii_point_mark_kinsoku: ContextualKinsoku,
inline_object_kinsoku: ContextualKinsoku, unicode_punctuation_boundaries: UnicodePunctuationBoundaries, western_bracket_cjk_inter_char_boundary_after_clusters: SortedSetTable<u32>, attached_inline_physical_boundary_after_clusters: SortedSetTable<u32>,
attached_inline_virtual_boundary_after_clusters: SortedMapTable<u32, u32>, attached_inline_virtual_sino_western_boundary_after_clusters: SortedSetTable<u32>, no_stretch_boundary_clusters: SortedSetTable<u32>, no_stretch_boundary_after_clusters: SortedSetTable<u32>,
technical_boundary_after_clusters: SortedMapTable<u32, ProgressiveBreakTier>, emergency_tracking_boundary_after_clusters: SortedMapTable<u32, String>, progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>, line_solution: LineSolution) -> Self {
        Self {
            metric_decisions,
            metric_decision_by_range,
            base_ascent,
            base_descent,
            base_box_descent,
            base_face_height,
            existing_interline_space,
            ruby_extent,
            base_line_metrics,
            line_spacing_decision,
            block_indent,
            first_line_indent,
            first_line_indent_decision,
            kinsoku_decision,
            ascii_point_mark_kinsoku,
            inline_object_kinsoku,
            unicode_punctuation_boundaries,
            western_bracket_cjk_inter_char_boundary_after_clusters,
            attached_inline_physical_boundary_after_clusters,
            attached_inline_virtual_boundary_after_clusters,
            attached_inline_virtual_sino_western_boundary_after_clusters,
            no_stretch_boundary_clusters,
            no_stretch_boundary_after_clusters,
            technical_boundary_after_clusters,
            emergency_tracking_boundary_after_clusters,
            progressive_break_opportunities,
            line_solution,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineBreakPlanningStageResult(",
            "metricDecisions=",
            {
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
    },
            ", ",
            "metricDecisionByRange=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.metric_decision_by_range).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", map.key_at(i).to_string(), map.value_at(i).to_string());
            i += 1;
        }
        out.push('}');
        out
    },
            ", ",
            "baseAscent=",
            self.base_ascent,
            ", ",
            "baseDescent=",
            self.base_descent,
            ", ",
            "baseBoxDescent=",
            self.base_box_descent,
            ", ",
            "baseFaceHeight=",
            self.base_face_height,
            ", ",
            "existingInterlineSpace=",
            self.existing_interline_space,
            ", ",
            "rubyExtent=",
            self.ruby_extent,
            ", ",
            "baseLineMetrics=",
            (self.base_line_metrics).clone().to_string(),
            ", ",
            "lineSpacingDecision=",
            (match &(self.line_spacing_decision) { None => "null".to_string(), Some(__option) => __option.to_string() }),
            ", ",
            "blockIndent=",
            self.block_indent,
            ", ",
            "firstLineIndent=",
            self.first_line_indent,
            ", ",
            "firstLineIndentDecision=",
            (self.first_line_indent_decision).clone().to_string(),
            ", ",
            "kinsokuDecision=",
            (self.kinsoku_decision).clone().to_string(),
            ", ",
            "asciiPointMarkKinsoku=",
            (self.ascii_point_mark_kinsoku).clone().to_string(),
            ", ",
            "inlineObjectKinsoku=",
            (self.inline_object_kinsoku).clone().to_string(),
            ", ",
            "unicodePunctuationBoundaries=",
            (self.unicode_punctuation_boundaries).clone().to_string(),
            ", ",
            "westernBracketCjkInterCharBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.western_bracket_cjk_inter_char_boundary_after_clusters).clone();
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "attachedInlinePhysicalBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.attached_inline_physical_boundary_after_clusters).clone();
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "attachedInlineVirtualBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.attached_inline_virtual_boundary_after_clusters).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", crate::runtime::int_text::IntText::int_text(map.key_at(i)), crate::runtime::int_text::IntText::int_text(map.value_at(i)));
            i += 1;
        }
        out.push('}');
        out
    },
            ", ",
            "attachedInlineVirtualSinoWesternBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.attached_inline_virtual_sino_western_boundary_after_clusters).clone();
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "noStretchBoundaryClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.no_stretch_boundary_clusters).clone();
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "noStretchBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.no_stretch_boundary_after_clusters).clone();
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "technicalBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.technical_boundary_after_clusters).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", crate::runtime::int_text::IntText::int_text(map.key_at(i)), map.value_at(i).name());
            i += 1;
        }
        out.push('}');
        out
    },
            ", ",
            "emergencyTrackingBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.emergency_tracking_boundary_after_clusters).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", crate::runtime::int_text::IntText::int_text(map.key_at(i)), map.value_at(i));
            i += 1;
        }
        out.push('}');
        out
    },
            ", ",
            "progressiveBreakOpportunities=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.progressive_break_opportunities).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", crate::runtime::int_text::IntText::int_text(map.key_at(i)), map.value_at(i).to_string());
            i += 1;
        }
        out.push('}');
        out
    },
            ", ",
            "lineSolution=",
            (self.line_solution).clone().to_string(),
            ")"
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct IntervalOverlapIndex {
    pub(crate) by_start: Vec<TextRange>,
    pub(crate) prefix_max_end: Vec<u32>,
}

impl IntervalOverlapIndex {
    pub fn new(ranges: Vec<TextRange>) -> Self {
    let mut prefix_max_end = Vec::new();
        let capacity = ranges.len();
        let mut sorted = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            sorted.push((ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let mut r_idx = 1u32;
        while (i32::from_ne_bytes((r_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((sorted.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let curr = (sorted[usize::try_from(r_idx).unwrap_or(0)]).clone();
            let mut j = r_idx;
            while (j) > (0) && (i32::from_ne_bytes((sorted[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)].start).to_ne_bytes())) > (i32::from_ne_bytes((curr.start).to_ne_bytes())) {
                sorted[usize::try_from(j).unwrap_or(0)] = (sorted[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]).clone();
                j = u32::wrapping_sub(j, 1);
            }
            sorted[usize::try_from(j).unwrap_or(0)] = curr;
            r_idx = u32::wrapping_add(r_idx, 1);
        }
        let mut running = 2147483648u32;
        for i in 0..match u32::try_from(sorted.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes((sorted[usize::try_from(i).unwrap_or(0)].end).to_ne_bytes()) > (i32::from_ne_bytes((running).to_ne_bytes())) {
                running = sorted[usize::try_from(i).unwrap_or(0)].end;
            }
            prefix_max_end.push(running);
        }
        Self {
            by_start: sorted,
            prefix_max_end: prefix_max_end,
        }
    }

    pub fn overlaps(&self, start: u32, end_exclusive: u32) -> bool {
        if u32::try_from(((self.by_start).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return false;
        }
        let mut low = 0u32;
        let mut high = u32::try_from(((self.by_start).clone().len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i32::from_ne_bytes((low).to_ne_bytes())) < (i32::from_ne_bytes((high).to_ne_bytes())) {
            let mid = u32::wrapping_add(low, high) >> 1;
            if i32::from_ne_bytes((self.by_start[usize::try_from(mid).unwrap_or(0)].start).to_ne_bytes()) < (i32::from_ne_bytes((end_exclusive).to_ne_bytes())) {
                low = u32::wrapping_add(mid, 1);
            } else {
                high = mid;
            }
        }
        return (i32::from_ne_bytes((low).to_ne_bytes())) > (0) && ({ let v: u32 = self.prefix_max_end[usize::try_from(u32::wrapping_sub(low, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > (i32::from_ne_bytes((start).to_ne_bytes()));
    }
}

#[derive(Clone, Copy)]
pub struct LineBreakPlanningStage;

impl LineBreakPlanningStage {
    pub(crate) fn line_break_planning_stage_is_hangable_punctuation(str: &str) -> bool {
        return str == "、" || str == "，" || str == "。";
    }

    pub(crate) fn line_break_planning_stage_is_whitespace_only(str: &str) -> bool {
    let __units = u_string::units(&str);
    let __count = u_string::unit_count(&str);
        if __count == 0 {
            return false;
        }
        for i in 0..match u32::try_from(u_string::unit_count(&(str))) { Ok(value) => value, Err(_) => u32::MAX } {
            if !ParagraphShapingStage::paragraph_shaping_stage_is_whitespace(*(u_string::unit_at_from(&__units, i)).as_ref().unwrap()) {
                return false;
            }
        }
        return true;
    }

    pub(crate) fn line_break_planning_stage_containing_cluster_metric_decisions(clusters: &Vec<Cluster>, decisions: &Vec<ClusterMetricDecision>) -> Vec<Option<ClusterMetricDecision>> {
        let mut item_index = 0u32;
        let mut result: Vec<Option<ClusterMetricDecision>> = Vec::new();
        for cluster in clusters {
            while (i32::from_ne_bytes((item_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) &&
(i32::from_ne_bytes((((decisions[usize::try_from(item_index).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes(((cluster.range).clone().start).to_ne_bytes()) {
                item_index = u32::wrapping_add(item_index, 1);
            }
            if i32::from_ne_bytes((item_index).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                let item = (decisions[usize::try_from(item_index).unwrap_or(0)]).clone();
                if i32::from_ne_bytes(((cluster.range).clone().start).to_ne_bytes()) >= i32::from_ne_bytes(((item.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes(((cluster.range).clone().end).to_ne_bytes())) <=
i32::from_ne_bytes(((item.range).clone().end).to_ne_bytes()) {
                    result.push(Some(item));
                } else {
                    result.push(None);
                }
            } else {
                result.push(None);
            }
        }
        return result;
    }

    pub fn line_break_planning_stage_plan_paragraph_lines(engine: ExplainableStubParagraphLayoutEngine, prep: ParagraphLayoutPrep) -> Result<LineBreakPlanningStageResult, TextRangeError> {
        let mut metric_cluster_index = 0u32;
        let capacity = prep.font_decisions.len();
        let mut metric_decisions = Vec::with_capacity(capacity);
        for dec_idx in 0..match u32::try_from(prep.font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let decision = (prep.font_decisions[usize::try_from(dec_idx).unwrap_or(0)]).clone();
            while (i32::from_ne_bytes((metric_cluster_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((prep.natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) &&
(i32::from_ne_bytes((((prep.natural_clusters[usize::try_from(metric_cluster_index).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes(((decision.range).clone().start).to_ne_bytes()) {
                metric_cluster_index = u32::wrapping_add(metric_cluster_index, 1);
            }
            let mut text_buf_b = String::new();
            while (i32::from_ne_bytes((metric_cluster_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((prep.natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) &&
(i32::from_ne_bytes((((prep.natural_clusters[usize::try_from(metric_cluster_index).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes())) < (i32::from_ne_bytes(((decision.range).clone().end).to_ne_bytes())) {
                let cluster = (prep.natural_clusters[usize::try_from(metric_cluster_index).unwrap_or(0)]).clone();
                if i32::from_ne_bytes(((cluster.range).clone().start).to_ne_bytes()) < (i32::from_ne_bytes(((decision.range).clone().start).to_ne_bytes())) || (i32::from_ne_bytes(((cluster.range).clone().end).to_ne_bytes())) >
(i32::from_ne_bytes(((decision.range).clone().end).to_ne_bytes())) {
                    return Err(TextRangeError::Message { text: format!("{}{}{}{}",
            "Shaped cluster ",
            (cluster.range).clone().to_string(),
            " crosses font decision ",
            (decision.range).clone().to_string()
        ).to_string() });
                }
                {
                    let x = (cluster.display_text).to_string().clone();
                    text_buf_b += &(x.to_string());
                }
                metric_cluster_index = u32::wrapping_add(metric_cluster_index, 1);
            }
            let buf_str = (text_buf_b).clone();
            let displayed_face_selection_text = if i32::from_ne_bytes((u_string::unit_count(&(buf_str))).to_ne_bytes()) > (0) { buf_str.to_string() } else { u_string::substring(&(prep.text).to_string(), i32::from_ne_bytes(((decision.range).clone().start).to_ne_bytes()),
i32::from_ne_bytes(((decision.range).clone().end).to_ne_bytes())).to_string() };
            let dec_style = (prep.style_at)((decision.range).clone().start);
            let capacity = dec_style.font_families.len();
            let mut font_families_copy = Vec::with_capacity(capacity);
            for f in 0..match u32::try_from(dec_style.font_families.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                font_families_copy.push((dec_style.font_families[usize::try_from(f).unwrap_or(0)]).clone());
            }
            let request = FontMetricsRequest::new(((decision.candidate).clone().key).to_string().as_str(), (prep.font_size_at)((decision.range).clone().start), decision.role, (((prep.input).clone().text_style).clone().locale).to_string().as_str(),
Some((font_families_copy).clone()), Some(dec_style.font_weight), Some(dec_style.italic), Some((displayed_face_selection_text).to_string()));
            let raw_metrics = engine.font_metrics_resolver.resolve((request).clone())?;
            let layout_metrics = engine.font_metrics_normalizer.normalize(FontMetricsNormalizationInput::new((request).clone(), (raw_metrics).clone()));
            metric_decisions.push(ClusterMetricDecision::new((decision.range).clone(), u_string::substring(&(prep.text).to_string(), i32::from_ne_bytes(((decision.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((decision.range).clone().end).to_ne_bytes())).as_str(),
(request).clone(), (raw_metrics).clone(), (layout_metrics).clone()));
        }
        let mut ideographic_decisions: Vec<ClusterMetricDecision> = Vec::new();
        for i in 0..match u32::try_from(metric_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if metric_decisions[usize::try_from(i).unwrap_or(0)].clone().layout_metrics.clone().metric_box == MetricBox::IdeographicEmBox {
                ideographic_decisions.push((metric_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let base_metric_decisions = if i32::from_ne_bytes((u32::try_from((ideographic_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) { ideographic_decisions } else { (metric_decisions).clone() };
        let mut max_ascent: Option<f64> = None;
        let mut max_descent: Option<f64> = None;
        for i in 0..match u32::try_from(base_metric_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let asc = ((base_metric_decisions[usize::try_from(i).unwrap_or(0)]).clone().layout_metrics).clone().ascent;
            let dsc = ((base_metric_decisions[usize::try_from(i).unwrap_or(0)]).clone().layout_metrics).clone().descent;
            if match &(max_ascent) { None => true, Some(__option1) => asc > (*__option1) } {
                max_ascent = Some(asc);
            }
            if match &(max_descent) { None => true, Some(__option2) => dsc > (*__option2) } {
                max_descent = Some(dsc);
            }
        }
        let base_ascent = match &(max_ascent) { Some(__option3) => *__option3, None => prep.font_size * 0.88f64 };
        let base_descent = match &(max_descent) { Some(__option4) => *__option4, None => prep.font_size * 0.12f64 };
        let mut base_ref_metrics: Option<LayoutFontMetrics> = None;
        for i in 0..match u32::try_from(metric_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if metric_decisions[usize::try_from(i).unwrap_or(0)].clone().layout_metrics.clone().metric_box == MetricBox::IdeographicEmBox && ((metric_decisions[usize::try_from(i).unwrap_or(0)]).clone().request).clone().font_size == prep.font_size {
                base_ref_metrics = Some(((metric_decisions[usize::try_from(i).unwrap_or(0)]).clone().layout_metrics).clone());
                break;
            }
        }
        let base_box_descent = match &(base_ref_metrics) { Some(__option5) => __option5.descent, None => base_descent };
        let mut max_ruby_extent = 0.0f64;
        for i in 0..u32::from_ne_bytes((prep.ruby_font_geometry_by_span.size()).to_ne_bytes()) {
            let geom = prep.ruby_font_geometry_by_span.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if geom.required_extent > (max_ruby_extent) {
                max_ruby_extent = geom.required_extent;
            }
        }
        let ruby_extent = max_ruby_extent;
        let interlinear_spacing_floor = if u32::try_from(((prep.input).clone().decorations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { 0.0f64 } else { 0.5f64 * prep.font_size };
        let default_body_line_height = prep.font_size * 1.5f64;
        let base_line_metrics = LineGeometryStageFns::line_geometry_stage_fns_line_metrics(&metric_decisions, ((prep.input).clone().paragraph_style).clone().line_height, default_body_line_height, Some(interlinear_spacing_floor));
        let containing_decisions = LineBreakPlanningStage::line_break_planning_stage_containing_cluster_metric_decisions(&prep.natural_clusters, &metric_decisions);
        let mut metric_decision_by_range_builder: SortedMapTableBuilder<TextRange, ClusterMetricDecision> = SortedTable::sorted_table_map_builder::<TextRange, ClusterMetricDecision>(Arc::new(compare_text_range));
        for i in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (containing_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            match &(d) {
                Some(__option6) => {
                    let decision = (*__option6).clone();
                    metric_decision_by_range_builder.put(&(((prep.natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone()), &(decision));
                }
                None => {
                }
            }
        }
        let metric_decision_by_range: SortedMapTable<TextRange, ClusterMetricDecision> = metric_decision_by_range_builder.clone().build();
        let base_face_height = base_ascent + base_descent;
        let mut existing_interline_space = base_line_metrics.height - base_face_height;
        if existing_interline_space < (0.0f64) {
            existing_interline_space = 0.0f64;
        }
        let mut line_spacing_decision: Option<LineSpacingDecisionInfo> = None;
        if base_line_metrics.height > (0.0f64) {
            let natural = base_line_metrics.height - base_line_metrics.extra_leading;
            let requested = ((prep.input).clone().paragraph_style).clone().line_height;
            let req_or_default = match &(requested) { Some(__option7) => *__option7, None => default_body_line_height };
            let mark_floor_binds = interlinear_spacing_floor > (0.0f64) && (natural + interlinear_spacing_floor) > (req_or_default + 0.001f64);
            let mut reason = "CjkBodyLineHeightDefault".to_string();
            if match &(requested) { Some(__option9) => !mark_floor_binds, None => false } {
                reason = "ExplicitLineHeight".to_string();
            } else {
                if mark_floor_binds {
                    reason = "InterlinearMarkLineSpacingFloor".to_string();
                }
            }
            line_spacing_decision = Some(LineSpacingDecisionInfo::new(natural, requested, base_line_metrics.height, interlinear_spacing_floor, mark_floor_binds, reason.as_str()).clone());
        }
        let mut explicit_indent_em: Option<f64> = None;
        let f_indent = ((prep.input).clone().paragraph_style).clone().first_line_indent;
        match &(f_indent) {
            Some(__option10) => {
                explicit_indent_em = Some(__option10.to_px(1.0f64));
            }
            None => {
            }
        }
        let indent_policy = (((prep.input).clone().paragraph_style).clone().first_line_indent_policy).clone();
        let block_indent = ((prep.input).clone().paragraph_style).clone().block_indent.to_px(prep.font_size);
        let resolved_indent_em = match &(explicit_indent_em) { Some(__option11) => *__option11, None => indent_policy.resolve_em(prep.measure_em) };
        let mut first_line_indent = block_indent + resolved_indent_em * prep.font_size;
        if first_line_indent < (0.0f64) {
            first_line_indent = 0.0f64;
        }
        let first_line_indent_decision = FirstLineIndentDecisionInfo::new(if explicit_indent_em.is_some() { "Explicit".to_string() } else { "MeasureAdaptiveFirstLineIndent".to_string() }.as_str(), prep.measure_em, indent_policy.short_below_em, resolved_indent_em);
        let kinsoku_decision = KinsokuDecisionInfo::new(prep.measure_em, (prep.resolved_kinsoku).clone().level.name().to_string().as_str(), (prep.resolved_kinsoku).clone().hanging.name().to_string().as_str(), ((prep.resolved_kinsoku).clone().reason).to_string().as_str());
        let mut hangable_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        if prep.resolved_kinsoku.clone().hanging == HangingPunctuationStyle::PauseStops {
            for idx in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if LineBreakPlanningStage::line_break_planning_stage_is_hangable_punctuation(((prep.natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone().display_text).to_string().as_str()) {
                    hangable_clusters_builder.put(&(idx));
                }
            }
        }
        let hangable_clusters: SortedSetTable<u32> = hangable_clusters_builder.clone().build();
        let ascii_point_mark_kinsoku = PunctuationGeometryStage::punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(&prep.natural_clusters, &prep.cluster_roles, &prep.clusters, (prep.resolved_kinsoku).clone().level, prep.measure - block_indent, prep.measure -
first_line_indent)?;
        let inline_object_kinsoku = PunctuationGeometryStage::punctuation_geometry_stage_inline_object_attached_kinsoku(&prep.natural_clusters, &prep.inline_object_attached_marks, &prep.clusters, (prep.resolved_kinsoku).clone().level, prep.measure - block_indent, prep.measure -
first_line_indent)?;
        let mut resolved_hangable_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((hangable_clusters.size()).to_ne_bytes()) {
            resolved_hangable_clusters_builder.put(&(hangable_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        for i in 0..u32::from_ne_bytes((ascii_point_mark_kinsoku.impossible_measure_hang_eligible_clusters.size()).to_ne_bytes()) {
            resolved_hangable_clusters_builder.put(&(ascii_point_mark_kinsoku.impossible_measure_hang_eligible_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        for i in 0..u32::from_ne_bytes((inline_object_kinsoku.impossible_measure_hang_eligible_clusters.size()).to_ne_bytes()) {
            resolved_hangable_clusters_builder.put(&(inline_object_kinsoku.impossible_measure_hang_eligible_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        let resolved_hangable_clusters: SortedSetTable<u32> = resolved_hangable_clusters_builder.clone().build();
        let unicode_punctuation_boundaries = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries((prep.text).to_string().as_str(), &prep.natural_clusters, &prep.cluster_roles, &prep.quote_pairs)?;
        let inline_attachments = prep.natural_inline_attachments.clone();
        let western_bracket_boundaries: SortedSetTable<u32> = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_western_bracket_cjk_inter_char_boundaries((prep.text).to_string().as_str(), &prep.natural_clusters, &prep.cluster_roles)?;
        let attached_inline_inter_char_boundaries = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries((prep.text).to_string().as_str(), &prep.natural_clusters, &prep.cluster_roles,
&prep.east_asian_spacing_edges, (western_bracket_boundaries).clone(), &inline_attachments)?;
        let western_bracket_cjk_inter_char_boundary_after_clusters: SortedSetTable<u32> = attached_inline_inter_char_boundaries.ordinary_western_boundary_after_clusters.clone();
        let attached_inline_physical_boundary_after_clusters: SortedSetTable<u32> = attached_inline_inter_char_boundaries.suppressed_physical_boundary_after_clusters.clone();
        let attached_inline_virtual_boundary_after_clusters: SortedMapTable<u32, u32> = attached_inline_inter_char_boundaries.virtual_boundary_after_clusters.clone();
        let attached_inline_virtual_sino_western_boundary_after_clusters: SortedSetTable<u32> = attached_inline_inter_char_boundaries.virtual_sino_western_boundary_after_clusters.clone();
        let mut attached_inline_forbidden_line_start_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for it in 0..match u32::try_from(inline_attachments.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if inline_attachments[usize::try_from(it).unwrap_or(0)] == InlineAttachment::Previous {
                attached_inline_forbidden_line_start_clusters_builder.put(&(it));
            }
        }
        let attached_inline_forbidden_line_start_clusters: SortedSetTable<u32> = attached_inline_forbidden_line_start_clusters_builder.clone().build();
        let mut forbidden_line_start_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for idx in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if attached_inline_forbidden_line_start_clusters.has(&(idx)) || prep.zero_width_break_clusters.has(&(idx)) || PunctuationGeometryStage::punctuation_geometry_stage_is_cjk_kinsoku_role(Some(prep.cluster_roles[usize::try_from(idx).unwrap_or(0)])) &&
prep.kinsoku_rule.forbidden_at_line_start((prep.natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone()) || unicode_punctuation_boundaries.forbidden_line_start_clusters.has(&(idx)) || ascii_point_mark_kinsoku.forbidden_line_start_clusters.has(&(idx)) ||
inline_object_kinsoku.forbidden_line_start_clusters.has(&(idx)) {
                forbidden_line_start_clusters_builder.put(&(idx));
            }
        }
        let forbidden_line_start_clusters: SortedSetTable<u32> = forbidden_line_start_clusters_builder.clone().build();
        let mut forbidden_line_end_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for idx in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if PunctuationGeometryStage::punctuation_geometry_stage_is_cjk_kinsoku_role(Some(prep.cluster_roles[usize::try_from(idx).unwrap_or(0)])) && prep.kinsoku_rule.forbidden_at_line_end((prep.natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone()) ||
unicode_punctuation_boundaries.forbidden_line_end_clusters.has(&(idx)) {
                forbidden_line_end_clusters_builder.put(&(idx));
            }
        }
        let forbidden_line_end_clusters: SortedSetTable<u32> = forbidden_line_end_clusters_builder.clone().build();
        let mut hyphen_break_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        if i32::from_ne_bytes((u32::from_ne_bytes((prep.hyphen_offsets.size()).to_ne_bytes())).to_ne_bytes()) > (0) {
            for it in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if prep.hyphen_offsets.has(&(((prep.natural_clusters[usize::try_from(it).unwrap_or(0)]).clone().range).clone().start)) {
                    hyphen_break_clusters_builder.put(&(it));
                }
            }
        }
        let hyphen_break_clusters: SortedSetTable<u32> = hyphen_break_clusters_builder.clone().build();
        let mut cluster_index_by_source_start_builder: SortedMapTableBuilder<u32, u32> = SortedTable::sorted_table_map_builder::<u32, u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for it in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            cluster_index_by_source_start_builder.put(&(((prep.natural_clusters[usize::try_from(it).unwrap_or(0)]).clone().range).clone().start), &(it));
        }
        let cluster_index_by_source_start: SortedMapTable<u32, u32> = cluster_index_by_source_start_builder.clone().build();
        let progressive_technical_whitespace_stretch_capacity = engine.justifier.progressive_technical_whitespace_stretch_capacity(prep.font_size);
        let mut progressive_break_opportunities_builder: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((prep.progressive_break_offsets.size()).to_ne_bytes()) {
            let source_offset = prep.progressive_break_offsets.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let opportunity = prep.progressive_break_offsets.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if cluster_index_by_source_start.has(&(source_offset)) {
                let cluster_index = (cluster_index_by_source_start.get(&(source_offset))).unwrap();
                let opp = if opportunity.tier == ProgressiveBreakTier::Whitespace { ProgressiveBreakOpportunity::new(opportunity.tier, (opportunity.span_range).clone(), Some(progressive_technical_whitespace_stretch_capacity)) } else { opportunity };
                progressive_break_opportunities_builder.put(&(cluster_index), &(opp));
            }
        }
        let progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity> = progressive_break_opportunities_builder.clone().build();
        let mut progressive_technical_ranges: Vec<TextRange> = Vec::new();
        for i in 0..match u32::try_from(((prep.input).clone().content).clone().line_break_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if prep.input.clone().content.clone().line_break_spans[usize::try_from(i).unwrap_or(0)].policy == LineBreakPolicy::ProgressiveTechnical {
                progressive_technical_ranges.push(((((prep.input).clone().content).clone().line_break_spans[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
            }
        }
        let progressive_technical_overlap = IntervalOverlapIndex::new(progressive_technical_ranges.to_vec());
        let raw_number_symbol_ranges = NumberSymbolCohesion::number_symbol_cohesion_unbreakable_ranges((prep.text).to_string().as_str());
        let mut number_symbol_cluster_ranges: Vec<IntRange> = Vec::new();
        for source_range in &raw_number_symbol_ranges {
            if !progressive_technical_overlap.overlaps(source_range.start, u32::wrapping_add(source_range.end, 1)) {
                let idx_range = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&prep.natural_clusters, TextRange::new(source_range.start, u32::wrapping_add(source_range.end, 1))?);
                match &(idx_range) {
                    Some(__option13) => {
                        number_symbol_cluster_ranges.push((__option13).clone());
                    }
                    None => {
                    }
                }
            }
        }
        let mut number_symbol_unbreakable_ranges: Vec<IntRange> = Vec::new();
        for idx_range in &number_symbol_cluster_ranges {
            let mut range_terms: Vec<f64> = vec![];
            for c_idx in idx_range.start..u32::wrapping_add(idx_range.end, 1) {
                range_terms.push(prep.natural_clusters[usize::try_from(c_idx).unwrap_or(0)].advance);
            }
            if AccurateSum::accurate_sum_of(&range_terms) <= prep.measure {
                number_symbol_unbreakable_ranges.push(idx_range.clone());
            }
        }
        let mut no_stretch_boundary_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for idx in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cls = if prep.atom_class_by_range.has(&(((prep.natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone().range).clone())) { prep.atom_class_by_range.get(&(((prep.natural_clusters[usize::try_from(idx).unwrap_or(0)]).clone().range).clone())) } else { None };
            if cls == Some(PunctuationClass::Connector) || cls == Some(PunctuationClass::Solidus) || cls == Some(PunctuationClass::Dash) || cls == Some(PunctuationClass::Ellipsis) {
                no_stretch_boundary_clusters_builder.put(&(idx));
            }
        }
        let no_stretch_boundary_clusters: SortedSetTable<u32> = no_stretch_boundary_clusters_builder.clone().build();
        let mut no_stretch_boundary_after_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for range in &number_symbol_cluster_ranges {
            for c_idx in range.start..range.end {
                no_stretch_boundary_after_clusters_builder.put(&(c_idx));
            }
        }
        for i in 0..u32::from_ne_bytes((prep.inline_object_attachment_no_stretch_boundaries.size()).to_ne_bytes()) {
            no_stretch_boundary_after_clusters_builder.put(&(prep.inline_object_attachment_no_stretch_boundaries.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        let no_stretch_boundary_after_clusters: SortedSetTable<u32> = no_stretch_boundary_after_clusters_builder.clone().build();
        let mut technical_boundary_after_clusters_builder: SortedMapTableBuilder<u32, ProgressiveBreakTier> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakTier>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((progressive_break_opportunities.size()).to_ne_bytes()) {
            let right_index = progressive_break_opportunities.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let opportunity = progressive_break_opportunities.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if opportunity.tier == ProgressiveBreakTier::Whitespace {
                technical_boundary_after_clusters_builder.put(&(u32::wrapping_sub(right_index, 1)), &(opportunity.tier));
            }
        }
        let technical_boundary_after_clusters: SortedMapTable<u32, ProgressiveBreakTier> = technical_boundary_after_clusters_builder.clone().build();
        let capacity = prep.natural_clusters.len();
        let mut boundary_eligible = Vec::with_capacity(capacity);
        for _ in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            boundary_eligible.push(false);
        }
        for left_index in 0..u32::try_from((prep.natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
            let right_index = u32::wrapping_add(left_index, 1);
            let left = (prep.natural_clusters[usize::try_from(left_index).unwrap_or(0)]).clone();
            let right = (prep.natural_clusters[usize::try_from(right_index).unwrap_or(0)]).clone();
            if left.range.clone().end != (right.range).clone().start {
                continue;
            }
            if prep.inline_object_by_cluster_index.has(&(left_index)) || prep.inline_object_by_cluster_index.has(&(right_index)) || prep.zero_width_break_clusters.has(&(left_index)) || prep.zero_width_break_clusters.has(&(right_index)) ||
prep.mandatory_break_clusters.has(&(left_index)) || prep.mandatory_break_clusters.has(&(right_index)) || u_string::unit_count(&((left.text).to_string())) == 0 || u_string::unit_count(&((right.text).to_string())) == 0 ||
LineBreakPlanningStage::line_break_planning_stage_is_whitespace_only((left.text).to_string().as_str()) || LineBreakPlanningStage::line_break_planning_stage_is_whitespace_only((right.text).to_string().as_str()) {
                continue;
            }
            { while boundary_eligible.len() <= usize::try_from(left_index).unwrap_or(0) { boundary_eligible.push(false); } boundary_eligible[usize::try_from(left_index).unwrap_or(0)] = true; };
        }
        let capacity = prep.natural_clusters.len();
        let mut added_keys = Vec::with_capacity(capacity);
        for _ in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            added_keys.push(false);
        }
        let mut emergency_tracking_boundary_after_clusters_builder: SortedMapTableBuilder<u32, String> = SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(prep.emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let decision = (prep.emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            let span = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&prep.natural_clusters, (decision.range).clone());
            if span.is_none() {
                continue;
            }
            for left_index in (span).as_ref().unwrap().start..(span).as_ref().unwrap().end {
                if !boundary_eligible[usize::try_from(left_index).unwrap_or(0)] || added_keys[usize::try_from(left_index).unwrap_or(0)] {
                    continue;
                }
                { while added_keys.len() <= usize::try_from(left_index).unwrap_or(0) { added_keys.push(false); } added_keys[usize::try_from(left_index).unwrap_or(0)] = true; };
                emergency_tracking_boundary_after_clusters_builder.put(&(left_index), &(decision.reason).to_string());
            }
        }
        let emergency_tracking_boundary_after_clusters: SortedMapTable<u32, String> = emergency_tracking_boundary_after_clusters_builder.clone().build();
        let mut adjustable_inline_boundary_right_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((prep.uniform_inline_object_boundary_after_clusters.size()).to_ne_bytes()) {
            let left_index = prep.uniform_inline_object_boundary_after_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let right_index = u32::wrapping_add(left_index, 1);
            if no_stretch_boundary_after_clusters.has(&(left_index)) || no_stretch_boundary_clusters.has(&(left_index)) || no_stretch_boundary_clusters.has(&(right_index)) {
            } else {
                adjustable_inline_boundary_right_clusters_builder.put(&(right_index));
            }
        }
        let adjustable_inline_boundary_right_clusters: SortedSetTable<u32> = adjustable_inline_boundary_right_clusters_builder.clone().build();
        let mut cjk_inter_char_boundaries_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for it in 1..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !attached_inline_physical_boundary_after_clusters.has(&(u32::wrapping_sub(it, 1))) && !no_stretch_boundary_after_clusters.has(&(u32::wrapping_sub(it, 1))) && prep.cluster_roles[usize::try_from(u32::wrapping_sub(it, 1)).unwrap_or(0)] == FontRole::CjkText &&
prep.cluster_roles[usize::try_from(it).unwrap_or(0)] == FontRole::CjkText {
                cjk_inter_char_boundaries_builder.put(&(it));
            }
        }
        for i in 0..u32::from_ne_bytes((adjustable_inline_boundary_right_clusters.size()).to_ne_bytes()) {
            cjk_inter_char_boundaries_builder.put(&(adjustable_inline_boundary_right_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        for i in 0..u32::from_ne_bytes((western_bracket_cjk_inter_char_boundary_after_clusters.size()).to_ne_bytes()) {
            cjk_inter_char_boundaries_builder.put(&(u32::wrapping_add(western_bracket_cjk_inter_char_boundary_after_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }), 1)));
        }
        for i in 0..u32::from_ne_bytes((attached_inline_virtual_boundary_after_clusters.size()).to_ne_bytes()) {
            cjk_inter_char_boundaries_builder.put(&(u32::wrapping_add(attached_inline_virtual_boundary_after_clusters.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }), 1)));
        }
        let cjk_inter_char_boundaries: SortedSetTable<u32> = cjk_inter_char_boundaries_builder.clone().build();
        let mut sino_western_boundaries_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for it in 1..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !attached_inline_physical_boundary_after_clusters.has(&(u32::wrapping_sub(it, 1))) && !no_stretch_boundary_after_clusters.has(&(u32::wrapping_sub(it, 1))) && PunctuationGeometryStage::punctuation_geometry_stage_is_east_asian_spacing_boundary_at(it,
&prep.natural_clusters, &prep.east_asian_spacing_edges) {
                sino_western_boundaries_builder.put(&(it));
            }
        }
        for i in 0..u32::from_ne_bytes((attached_inline_virtual_sino_western_boundary_after_clusters.size()).to_ne_bytes()) {
            sino_western_boundaries_builder.put(&(u32::wrapping_add(attached_inline_virtual_sino_western_boundary_after_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }), 1)));
        }
        let sino_western_boundaries: SortedSetTable<u32> = sino_western_boundaries_builder.clone().build();
        let virtual_boundaries = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&inline_attachments);
        let capacity = virtual_boundaries.len();
        let mut attached_inline_unbreakable_ranges = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(virtual_boundaries.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let boundary = (virtual_boundaries[usize::try_from(i).unwrap_or(0)]).clone();
            attached_inline_unbreakable_ranges.push(IntRange::new(boundary.previous_cluster_index, boundary.attached_cluster_range.end));
        }
        let mut all_unbreakables: Vec<IntRange> = Vec::new();
        for i in 0..match u32::try_from((prep.input).clone().decorations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((prep.input).clone().decorations[usize::try_from(i).unwrap_or(0)]).clone();
            if d.kind == DecorationKind::Mourning {
                let r = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&prep.natural_clusters, (d.range).clone());
                match &(r) {
                    Some(__option14) => {
                        all_unbreakables.push((__option14).clone());
                    }
                    None => {
                    }
                }
            }
        }
        for i in 0..match u32::try_from(prep.pinyin_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let r = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&prep.natural_clusters, ((prep.pinyin_spans[usize::try_from(i).unwrap_or(0)]).clone().base_range).clone());
            match &(r) {
                Some(__option15) => {
                    all_unbreakables.push((__option15).clone());
                }
                None => {
                }
            }
        }
        for i in 0..match u32::try_from(attached_inline_unbreakable_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_unbreakables.push((attached_inline_unbreakable_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(number_symbol_unbreakable_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_unbreakables.push((number_symbol_unbreakable_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(unicode_punctuation_boundaries.unbreakable_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_unbreakables.push((unicode_punctuation_boundaries.unbreakable_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(ascii_point_mark_kinsoku.unbreakable_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_unbreakables.push((ascii_point_mark_kinsoku.unbreakable_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(inline_object_kinsoku.unbreakable_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_unbreakables.push((inline_object_kinsoku.unbreakable_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(prep.inline_object_boundary_unbreakable_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all_unbreakables.push((prep.inline_object_boundary_unbreakable_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let unbreakable_ranges = UnbreakableRanges::new(all_unbreakables.to_vec());
        let capacity = ascii_point_mark_kinsoku.extendable_hang_ranges.len();
        let mut combined_extendable_hang_ranges = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(ascii_point_mark_kinsoku.extendable_hang_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            combined_extendable_hang_ranges.push((ascii_point_mark_kinsoku.extendable_hang_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        for i in 0..match u32::try_from(inline_object_kinsoku.extendable_hang_ranges.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            combined_extendable_hang_ranges.push((inline_object_kinsoku.extendable_hang_ranges[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let mut line_solution = LineSolution::new(Some(vec![]), Some(0 as f64))?;
        if i32::from_ne_bytes((u_string::unit_count(&((prep.text).to_string()))).to_ne_bytes()) > (0) {
            let mut line_adjustment_compress_bias = 0.0f64;
            if prep.adjustment_style.clone().line_adjustment == LineAdjustmentStrategy::PushInFirst {
                line_adjustment_compress_bias = 1000000.0f64;
            } else {
                if prep.adjustment_style.clone().line_adjustment == LineAdjustmentStrategy::PushOutFirst {
                    line_adjustment_compress_bias = 0.5f64;
                }
            }
            line_solution = engine.line_breaker.break_lines(&prep.natural_clusters, &prep.clusters, prep.measure - block_indent, Some((prep.shrink_opportunities).clone()), Some((unbreakable_ranges).clone()), Some(first_line_indent - block_indent),
Some((resolved_hangable_clusters).clone()), Some((combined_extendable_hang_ranges).clone()), Some((forbidden_line_start_clusters).clone()), Some((forbidden_line_end_clusters).clone()), Some((hyphen_break_clusters).clone()), Some((cjk_inter_char_boundaries).clone()), Some(0.5f64 *
prep.font_size), Some((sino_western_boundaries).clone()), Some(0.25f64 * prep.font_size), Some((prep.adjustment_style).clone().line_adjustment != LineAdjustmentStrategy::PushOutOnly), Some(line_adjustment_compress_bias), Some((prep.mandatory_break_clusters).clone()),
Some((prep.zero_width_break_clusters).clone()), Some((progressive_break_opportunities).clone()))?;
        }
        return Ok(LineBreakPlanningStageResult::new(metric_decisions.to_vec(), (metric_decision_by_range).clone(), base_ascent, base_descent, base_box_descent, base_face_height, existing_interline_space, ruby_extent, (base_line_metrics).clone(), (line_spacing_decision).clone(),
block_indent, first_line_indent, (first_line_indent_decision).clone(), (kinsoku_decision).clone(), (ascii_point_mark_kinsoku).clone(), (inline_object_kinsoku).clone(), (unicode_punctuation_boundaries).clone(), (western_bracket_cjk_inter_char_boundary_after_clusters).clone(),
(attached_inline_physical_boundary_after_clusters).clone(), (attached_inline_virtual_boundary_after_clusters).clone(), (attached_inline_virtual_sino_western_boundary_after_clusters).clone(), (no_stretch_boundary_clusters).clone(), (no_stretch_boundary_after_clusters).clone(),
(technical_boundary_after_clusters).clone(), (emergency_tracking_boundary_after_clusters).clone(), (progressive_break_opportunities).clone(), (line_solution).clone()));
    }
}
