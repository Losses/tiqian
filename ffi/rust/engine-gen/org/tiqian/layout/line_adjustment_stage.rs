use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::line_end_punctuation_style::LineEndPunctuationStyle;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::contextual_kinsoku_decision_info::ContextualKinsokuDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_decision_info::InlineObjectDecisionInfo;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_debug_info::LineDebugInfo;
use crate::org::tiqian::core::line_edge_trim_decision_info::LineEdgeTrimDecisionInfo;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::max_lines_decision_info::MaxLinesDecisionInfo;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::size::Size;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::font::baseline_class::BaselineClass;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::annotation_geometry_stage::AnnotationGeometryStage;
use crate::org::tiqian::layout::annotation_geometry_stage::AnnotationGeometryStageResult;
use crate::org::tiqian::layout::annotation_geometry_stage::RubyFontGeometry;
use crate::org::tiqian::layout::justifier::JustificationPlan;
use crate::org::tiqian::layout::layout_debug_assembly::LayoutDebugAssembly;
use crate::org::tiqian::layout::layout_debug_assembly::LayoutDebugStageInput;
use crate::org::tiqian::layout::line_break_planning_stage::LineBreakPlanningStageResult;
use crate::org::tiqian::layout::line_break_planning_stage::ParagraphLayoutPrep;
use crate::org::tiqian::layout::line_geometry_stage::ClusterMetricDecision;
use crate::org::tiqian::layout::line_geometry_stage::LineBoxStageResult;
use crate::org::tiqian::layout::line_geometry_stage::LineGeometryStageFns;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStage;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::org::tiqian::layout::punctuation_geometry_stage::PunctuationGeometryStage;
use crate::org::tiqian::layout::punctuation_model::GlueKind;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageFinishParagraphLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
}

impl From<LineAdjustmentStageFinishParagraphLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageFinishParagraphLayoutFault) -> Self {
        match value {
            LineAdjustmentStageFinishParagraphLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageFinishParagraphLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageFinishParagraphLayoutFault) -> Self {
        match value {
            LineAdjustmentStageFinishParagraphLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageFinishParagraphLayoutFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: LineAdjustmentStageFinishParagraphLayoutFault) -> Self {
        match value {
            LineAdjustmentStageFinishParagraphLayoutFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageFinishParagraphLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageFinishParagraphLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageFinishParagraphLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageFinishParagraphLayoutFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for LineAdjustmentStageFinishParagraphLayoutFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        LineAdjustmentStageFinishParagraphLayoutFault::TextShaperShapeFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct LineAdjustmentStage;

impl LineAdjustmentStage {
    pub(crate) fn line_adjustment_stage_repair_option_name(repair: RepairOption) -> String {
        return match repair {
            RepairOption::PushIn { .. } => "PushIn".to_string(),
            RepairOption::Hang { .. } => "Hang".to_string(),
            RepairOption::CarryPrevious { .. } => "CarryPrevious".to_string(),
            RepairOption::CarryNext { .. } => "CarryNext".to_string(),
            RepairOption::LeaveRagged { .. } => "LeaveRagged".to_string(),
        };
    }

    pub(crate) fn line_adjustment_stage_line_hyphen_advance_at(line_index: u32, lines: &Vec<LineCandidate>, hyphen_offsets: SortedSetTable<u32>, natural_clusters: &Vec<Cluster>, hyphen_advance: f64) -> f64 {
        if u32::from_ne_bytes((hyphen_offsets.size()).to_ne_bytes()) == 0 || (i32::from_ne_bytes((line_index).to_ne_bytes())) >= i32::from_ne_bytes((u32::wrapping_sub(u32::try_from((lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).to_ne_bytes()) {
            return 0.0f64;
        }
        let next = (lines[usize::try_from(u32::wrapping_add(line_index, 1)).unwrap_or(0)]).clone();
        if next.cluster_range.get_is_empty() {
            return 0.0f64;
        }
        let next_first = next.cluster_range.start;
        return if hyphen_offsets.has(&(((natural_clusters[usize::try_from(next_first).unwrap_or(0)]).clone().range).clone().start)) { hyphen_advance } else { 0.0f64 };
    }

    pub(crate) fn line_adjustment_stage_renderable_glyph_run_clusters(clusters: &Vec<Cluster>, open_type_features_by_cluster_range: SortedMapTable<TextRange, Vec<String>>) -> Vec<Vec<Cluster>> {
        let mut renderable: Vec<Cluster> = Vec::new();
        for c in clusters {
            if i32::from_ne_bytes((u_string::unit_count(&((c.display_text).to_string()))).to_ne_bytes()) > (0) && !ParagraphShapingStage::paragraph_shaping_stage_is_inline_object_cluster((c).clone()) {
                renderable.push(c.clone());
            }
        }
        if u32::try_from((renderable.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let mut groups: Vec<Vec<Cluster>> = Vec::new();
        let mut current_group = vec![(renderable[0usize]).clone()];
        for i in 1..match u32::try_from(renderable.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let prev = (current_group[usize::try_from(u32::wrapping_sub(u32::try_from((current_group.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
            let curr = (renderable[usize::try_from(i).unwrap_or(0)]).clone();
            let prev_feat = if open_type_features_by_cluster_range.has(&((prev.range).clone())) { (open_type_features_by_cluster_range.get(&((prev.range).clone()))).unwrap() } else { vec![] };
            let curr_feat = if open_type_features_by_cluster_range.has(&((curr.range).clone())) { (open_type_features_by_cluster_range.get(&((curr.range).clone()))).unwrap() } else { vec![] };
            let mut same_features = u32::try_from((prev_feat.len()) & 0xFFFF_FFFF).unwrap_or(0) == u32::try_from((curr_feat.len()) & 0xFFFF_FFFF).unwrap_or(0);
            if same_features {
                for f in 0..match u32::try_from(prev_feat.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if prev_feat[usize::try_from(f).unwrap_or(0)].clone() != (curr_feat[usize::try_from(f).unwrap_or(0)]).clone() {
                        same_features = false;
                        break;
                    }
                }
            }
            if prev.font_key.to_string() == (curr.font_key).to_string() && (prev.range).clone().end == (curr.range).clone().start && same_features {
                current_group.push(curr.clone());
            } else {
                groups.push(current_group.clone());
                current_group = vec![(curr).clone()];
            }
        }
        groups.push(current_group.clone());
        return groups;
    }

    pub(crate) fn line_adjustment_stage_center_dash_ink(glyphs: &Vec<Glyph>, cluster: Cluster, atom_class_by_range: SortedMapTable<TextRange, PunctuationClass>) -> Vec<Glyph> {
        if !atom_class_by_range.has(&((cluster.range).clone())) || atom_class_by_range.get(&((cluster.range).clone())) != Some(PunctuationClass::Dash) {
            return (*glyphs).clone();
        }
        if u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) != 1 {
            return (*glyphs).clone();
        }
        let glyph = (glyphs[0usize]).clone();
        let ink = glyph.bounds.clone();
        if ink.is_none() {
            return (*glyphs).clone();
        }
        let inset = (cluster.advance - ((ink).as_ref().unwrap().right - (ink).as_ref().unwrap().left)) / 2.0f64 - (ink).as_ref().unwrap().left;
        if inset <= 0.5f64 {
            return (*glyphs).clone();
        }
        return vec![
    (Glyph::new(glyph.id, (glyph.cluster_range).clone(), glyph.advance, Some(glyph.x + inset), Some(glyph.y), glyph.render_font_key.clone(), (glyph.bounds).clone(), glyph.halt_advance, glyph.halt_placement_x)).clone(),
];
    }

    pub(crate) fn line_adjustment_stage_build_line_boxes(input: LayoutInput, line_solution: LineSolution, trimmed_clusters: &Vec<Cluster>, final_clusters: &Vec<Cluster>, first_line_indent: f64, block_indent: f64, measure: f64, grid_body_offset: f64, line_baseline: &Vec<f64>,
line_top: &Vec<f64>, line_bottom: &Vec<f64>, hyphen_offsets: SortedSetTable<u32>, natural_clusters: &Vec<Cluster>, hyphen_advance: f64, hyphen_glyphs: &Vec<Glyph>, justification_plans: &Vec<Option<JustificationPlan>>) -> LineBoxStageResult {
        let mut laid_out_lines: Vec<LineBox> = Vec::new();
        for line_index in 0..match u32::try_from(line_solution.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line_candidate = (line_solution.lines[usize::try_from(line_index).unwrap_or(0)]).clone();
            let mut adjusted_terms: Vec<f64> = vec![];
            let mut visual_terms: Vec<f64> = vec![];
            if !line_candidate.cluster_range.get_is_empty() {
                for idx in line_candidate.cluster_range.start..u32::wrapping_add(line_candidate.cluster_range.end, 1) {
                    adjusted_terms.push(if line_candidate.hanging_cluster_indices.has(&(idx)) { 0.0f64 } else { trimmed_clusters[usize::try_from(idx).unwrap_or(0)].advance });
                    visual_terms.push(final_clusters[usize::try_from(idx).unwrap_or(0)].advance);
                }
            }
            let adjusted_width = AccurateSum::accurate_sum_of(&adjusted_terms);
            let visual_width = AccurateSum::accurate_sum_of(&visual_terms);
            let mut hanging_terms: Vec<f64> = vec![];
            for h_idx in 0..u32::from_ne_bytes((line_candidate.hanging_cluster_indices.size()).to_ne_bytes()) {
                hanging_terms.push(final_clusters[usize::try_from(line_candidate.hanging_cluster_indices.at({ let v: u32 = h_idx; i32::from_ne_bytes(v.to_ne_bytes()) })).unwrap_or(0)].advance);
            }
            let hanging_punctuation_advance = AccurateSum::accurate_sum_of(&hanging_terms);
            let mut has_drawable_content = false;
            if !line_candidate.cluster_range.get_is_empty() {
                for idx in line_candidate.cluster_range.start..u32::wrapping_add(line_candidate.cluster_range.end, 1) {
                    if i32::from_ne_bytes((u_string::unit_count(&(((final_clusters[usize::try_from(idx).unwrap_or(0)]).clone().display_text).to_string()))).to_ne_bytes()) > (0) {
                        has_drawable_content = true;
                        break;
                    }
                }
            }
            let mut base_indent = 0.0f64;
            if has_drawable_content {
                if line_candidate.cluster_range.start == 0 {
                    base_indent = first_line_indent;
                } else {
                    base_indent = block_indent;
                }
            }
            let line_hyphen_advance = LineAdjustmentStage::line_adjustment_stage_line_hyphen_advance_at(line_index, &line_solution.lines, (hyphen_offsets).clone(), &natural_clusters, hyphen_advance);
            let limit = measure - base_indent;
            let mut alignment_inset = 0.0f64;
            if line_candidate.end_reason != LineEndReason::AutoWrap {
                if input.paragraph_style.clone().last_line_alignment == LastLineAlignment::Center {
                    let diff = (limit - visual_width) / 2.0f64;
                    alignment_inset = if diff < (0.0f64) { 0.0f64 } else { diff };
                } else {
                    if input.paragraph_style.clone().last_line_alignment == LastLineAlignment::End {
                        let diff = limit - visual_width;
                        alignment_inset = if diff < (0.0f64) { 0.0f64 } else { diff };
                    }
                }
            }
            let repair_str = match &(line_candidate.repair) { Some(__option) => Some(format!("{}{}{}",
            LineAdjustmentStage::line_adjustment_stage_repair_option_name((*__option).clone()),
            ":",
            RepairOptions::repair_options_reason((*__option).clone())
        ).to_string()), None => None };
            let mut notes: Vec<String> = Vec::new();
            if line_candidate.cluster_range.get_is_empty() {
                notes.push(format!("{}{}{}",
            "line:",
            crate::runtime::int_text::IntText::int_text(line_index),
            ":clusters=empty"
        ));
            } else {
                notes.push(format!("{}{}{}{}{}{}",
            "line:",
            crate::runtime::int_text::IntText::int_text(line_index),
            ":clusters=",
            crate::runtime::int_text::IntText::int_text(line_candidate.cluster_range.start),
            "-",
            crate::runtime::int_text::IntText::int_text(line_candidate.cluster_range.end)
        ));
            }
            notes.push(format!("{}{}",
            "end:",
            line_candidate.end_reason.name()
        ));
            notes.push(format!("{}{}{}{}{}{}",
            "natural=",
            line_candidate.natural_width,
            ",adjusted=",
            line_candidate.adjusted_width,
            ",visual=",
            visual_width
        ));
            let plan_for_line = (justification_plans[usize::try_from(line_index).unwrap_or(0)]).clone();
            match &(plan_for_line) {
                Some(__option1) => {
                    if __option1.fallback_reason.is_some() {
                    notes.push(format!("{}{}",
            "justify-fallback:",
            match __option1.fallback_reason { Some(ref v) => v.to_string(), None => "null".to_string() }
        ));
                    }
                }
                None => {
                }
            }
            let h_glyphs = if line_hyphen_advance > (0.0f64) { (*hyphen_glyphs).clone() } else { vec![] };
            laid_out_lines.push(LineBox::new((line_candidate.source_range).clone(), (line_candidate.cluster_range).clone(), line_baseline[usize::try_from(line_index).unwrap_or(0)], line_top[usize::try_from(line_index).unwrap_or(0)],
line_bottom[usize::try_from(line_index).unwrap_or(0)], line_candidate.natural_width, adjusted_width, visual_width, Some(hanging_punctuation_advance), Some(grid_body_offset + base_indent + alignment_inset), Some(line_candidate.end_reason), Some(line_hyphen_advance),
Some((h_glyphs).clone()), LineDebugInfo::new(repair_str.clone(), Some((notes).clone()))));
        }
        let mut visible_lines: Vec<LineBox> = Vec::new();
        let max_lines = (input.constraints).clone().max_lines;
        let count = if i32::from_ne_bytes((u32::try_from((laid_out_lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (i32::from_ne_bytes((max_lines).to_ne_bytes())) { max_lines } else { u32::try_from((laid_out_lines.len()) & 0xFFFF_FFFF).unwrap_or(0) };
        for i in 0..count {
            visible_lines.push((laid_out_lines[usize::try_from(i).unwrap_or(0)]).clone());
        }
        let max_lines_decision = if i32::from_ne_bytes((u32::try_from((visible_lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((laid_out_lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
Some(MaxLinesDecisionInfo::new(u32::try_from((laid_out_lines.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from((visible_lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some("MaxLinesLineTruncation".to_string()))) } else { None };
        let capacity = visible_lines.len();
        let mut visible_line_ranges = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(visible_lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            visible_line_ranges.push((line_solution.lines[usize::try_from(i).unwrap_or(0)]).clone().cluster_range.clone());
        }
        return LineBoxStageResult::new(laid_out_lines.to_vec(), visible_lines.to_vec(), (max_lines_decision).clone(), visible_line_ranges.to_vec());
    }

    pub(crate) fn line_adjustment_stage_is_fixed_boundary(b: InlineObjectBoundaryAdjustment) -> bool {
        return !b.participates_in_uniform_stretch && b.preferred_stretch.is_none() && b.shrink_capacity == 0.0f64 && b.line_end_discardable_advance == 0.0f64 && !b.prevents_line_break;
    }

    pub fn line_adjustment_stage_resolve_annotation_geometry(engine: &mut ExplainableStubParagraphLayoutEngine, input: LayoutInput, font_size: f64, inline_object_by_cluster_index: SortedMapTable<u32, InlineObjectSpan>, line_solution: LineSolution, clreq_profile: ClreqProfile,
geometry_decisions: &Vec<ClusterGeometryDecisionInfo>, auto_space_decisions: &Vec<AutoSpaceDecisionInfo>, visible_line_ranges: &Vec<IntRange>, lines: &Vec<LineBox>, final_clusters: &Vec<Cluster>, cluster_roles: &Vec<FontRole>, justify_delta_by_cluster: SortedMapTable<u32, f64>,
ruby_and_bopomofo_spread: SortedMapTable<u32, f64>, metric_decisions: &Vec<ClusterMetricDecision>, pinyin_spans: &Vec<RubySpan>, natural_clusters: &Vec<Cluster>, ruby_font_geometry_by_span: SortedMapTable<RubySpan, RubyFontGeometry>, ruby_stack_gap: f64, base_ascent: f64,
ruby_font_size: f64, ruby_font_weight: u32, base_descent: f64, bopomofo_font_weight_at: Arc<dyn Fn(u32) -> u32 + Send + Sync>) -> Result<AnnotationGeometryStageResult, TextShaperShapeFault> {
        let capacity = usize::try_from(u32::from_ne_bytes((inline_object_by_cluster_index.size()).to_ne_bytes())).unwrap_or(0);
        let mut inline_object_decisions = Vec::with_capacity(capacity);
        for i in 0..u32::from_ne_bytes((inline_object_by_cluster_index.size()).to_ne_bytes()) {
            let cluster_index = inline_object_by_cluster_index.key_at(i32::from_ne_bytes((i).to_ne_bytes()));
            let inline_object = inline_object_by_cluster_index.value_at(i32::from_ne_bytes((i).to_ne_bytes()));
            let mut line_idx = 4294967295u32;
            for l in 0..match u32::try_from(line_solution.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let cr = (line_solution.lines[usize::try_from(l).unwrap_or(0)]).clone().cluster_range;
                if !cr.get_is_empty() && (i32::from_ne_bytes((cluster_index).to_ne_bytes())) >= i32::from_ne_bytes((cr.start).to_ne_bytes()) && (i32::from_ne_bytes((cluster_index).to_ne_bytes())) <= i32::from_ne_bytes((cr.end).to_ne_bytes()) {
                    line_idx = l;
                    break;
                }
            }
            let leading_preferred = (inline_object.leading_boundary).clone().preferred_stretch;
            let trailing_preferred = (inline_object.trailing_boundary).clone().preferred_stretch;
            let reason = if !LineAdjustmentStage::line_adjustment_stage_is_fixed_boundary((inline_object.leading_boundary).clone()) || !LineAdjustmentStage::line_adjustment_stage_is_fixed_boundary((inline_object.trailing_boundary).clone()) { "AdjustableInlineObject".to_string() }
else { "MeasurableOpaqueInlineObject".to_string() };
            inline_object_decisions.push(InlineObjectDecisionInfo::new((inline_object.range).clone(), inline_object.advance, inline_object.ascent, inline_object.descent, cluster_index, line_idx, Some((inline_object.leading_boundary).clone().participates_in_uniform_stretch), match
&(leading_preferred) { Some(__option2) => Some(__option2.kind.name().to_string()), None => None }.clone(), Some(match &(leading_preferred) { Some(__option4) => __option4.natural_width, None => 0.0f64 }), Some(match &(leading_preferred) { Some(__option6) => __option6.target_width,
None => 0.0f64 }), Some(match &(leading_preferred) { Some(__option8) => __option8.get_capacity(), None => 0.0f64 }), Some((inline_object.leading_boundary).clone().prevents_line_break), Some((inline_object.leading_boundary).clone().shrink_capacity),
Some((inline_object.leading_boundary).clone().line_end_discardable_advance), Some((inline_object.trailing_boundary).clone().participates_in_uniform_stretch), match &(trailing_preferred) { Some(__option10) => Some(__option10.kind.name().to_string()), None => None }.clone(),
Some(match &(trailing_preferred) { Some(__option12) => __option12.natural_width, None => 0.0f64 }), Some(match &(trailing_preferred) { Some(__option14) => __option14.target_width, None => 0.0f64 }), Some(match &(trailing_preferred) { Some(__option16) => __option16.get_capacity(),
None => 0.0f64 }), Some((inline_object.trailing_boundary).clone().prevents_line_break), Some((inline_object.trailing_boundary).clone().shrink_capacity), Some((inline_object.trailing_boundary).clone().line_end_discardable_advance), Some((reason).to_string())));
        }
        let decoration_decisions = AnnotationGeometryStage::annotation_geometry_stage_compute_decoration_decisions(&input.decorations, &visible_line_ranges, &lines, &final_clusters, &cluster_roles, (justify_delta_by_cluster).clone(), (ruby_and_bopomofo_spread).clone(),
&metric_decisions, font_size, (input.paragraph_style).clone().emphasis_dot_gap_em);
        let auto_space_gap_px = (clreq_profile.auto_space).clone().gap_em * font_size;
        let mut geometry_by_range_builder: SortedMapTableBuilder<TextRange, ClusterGeometryDecisionInfo> = SortedTable::sorted_table_map_builder::<TextRange, ClusterGeometryDecisionInfo>(Arc::new(compare_text_range));
        for gd in geometry_decisions {
            geometry_by_range_builder.put(&((gd.range).clone()), &(gd));
        }
        let geometry_by_range: SortedMapTable<TextRange, ClusterGeometryDecisionInfo> = geometry_by_range_builder.clone().build();
        let mut leading_gap_ranges_builder: SortedSetTableBuilder<TextRange> = SortedTable::sorted_table_set_builder::<TextRange>(Arc::new(compare_text_range));
        let mut trailing_gap_ranges_builder: SortedSetTableBuilder<TextRange> = SortedTable::sorted_table_set_builder::<TextRange>(Arc::new(compare_text_range));
        for ad in auto_space_decisions {
            if ad.side.to_string() == "leading" {
                leading_gap_ranges_builder.put(&((ad.cluster_range).clone()));
            } else {
                if ad.side.to_string() == "trailing" {
                    trailing_gap_ranges_builder.put(&((ad.cluster_range).clone()));
                }
            }
        }
        let leading_gap_ranges: SortedSetTable<TextRange> = leading_gap_ranges_builder.clone().build();
        let trailing_gap_ranges: SortedSetTable<TextRange> = trailing_gap_ranges_builder.clone().build();
        let decoration_segments = AnnotationGeometryStage::annotation_geometry_stage_compute_decoration_segments(&input.decorations, &visible_line_ranges, &lines, &final_clusters, (justify_delta_by_cluster).clone(), (geometry_by_range).clone(), (leading_gap_ranges).clone(),
(trailing_gap_ranges).clone(), auto_space_gap_px, font_size).map_err(|e| TextShaperShapeFault::TextRangeErrorFault(e))?;
        let ruby_decisions = AnnotationGeometryStage::annotation_geometry_stage_compute_ruby_decisions(&pinyin_spans, &visible_line_ranges, &lines, &final_clusters, &natural_clusters, &metric_decisions, (ruby_font_geometry_by_span).clone(), ruby_stack_gap, base_ascent,
ruby_font_size, ruby_font_weight, ((input.text_style).clone().locale).to_string().as_str());
        let mut bopomofo_spans: Vec<RubySpan> = Vec::new();
        for ri in 0..match u32::try_from(input.ruby_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let rs = (input.ruby_spans[usize::try_from(ri).unwrap_or(0)]).clone();
            if rs.kind == RubyKind::Bopomofo {
                bopomofo_spans.push(rs.clone());
            }
        }
        let bopomofo_decisions = AnnotationGeometryStage::annotation_geometry_stage_compute_bopomofo_decisions(engine, &bopomofo_spans, &visible_line_ranges, &lines, &final_clusters, &natural_clusters, base_ascent, base_descent, font_size, (bopomofo_font_weight_at).clone(),
(input.text_style).clone())?;
        return Ok(AnnotationGeometryStageResult::new(inline_object_decisions.to_vec(), decoration_decisions.to_vec(), decoration_segments.to_vec(), ruby_decisions.to_vec(), bopomofo_decisions.to_vec()));
    }

    pub(crate) fn line_adjustment_stage_trim_edge(line_source_range: TextRange, cluster_idx: u32, side: &str, natural_clusters: &Vec<Cluster>, auto_space_decisions: &Vec<AutoSpaceDecisionInfo>, auto_space_gap: f64, auto_space_edge_trims: &mut Vec<f64>, auto_space_edge_decisions:
&mut Vec<LineEdgeTrimDecisionInfo>) {
        let mut found_decision: Option<AutoSpaceDecisionInfo> = None;
        let c_range = ((natural_clusters[usize::try_from(cluster_idx).unwrap_or(0)]).clone().range).clone();
        for dec in auto_space_decisions {
            if dec.cluster_range.clone().start == c_range.start && (dec.cluster_range).clone().end == c_range.end && (dec.side).to_string() == side {
                found_decision = Some(dec.clone());
                break;
            }
        }
        match &(found_decision) {
            Some(__option18) => {
                auto_space_edge_trims[usize::try_from(cluster_idx).unwrap_or(0)] += auto_space_gap;
                auto_space_edge_decisions.push(LineEdgeTrimDecisionInfo::new((line_source_range).clone(), (__option18.cluster_range).clone(), side, auto_space_gap, 0.0f64, auto_space_gap, "TextAutoSpaceLineEdgeTrim"));
            }
            None => {
            }
        }
    }

    pub(crate) fn line_adjustment_stage_collapse_edge_space(line_source_range: TextRange, cluster_idx: u32, side: &str, natural_clusters: &Vec<Cluster>, inline_object_separator_space_trims: SortedMapTable<u32, f64>, auto_space_edge_trims: &mut Vec<f64>, auto_space_edge_decisions:
&mut Vec<LineEdgeTrimDecisionInfo>) {
        let cluster = (natural_clusters[usize::try_from(cluster_idx).unwrap_or(0)]).clone();
        if !PunctuationGeometryStage::punctuation_geometry_stage_is_space_run((cluster).clone()) {
            return;
        }
        if inline_object_separator_space_trims.has(&(cluster_idx)) {
            return;
        }
        let advance = cluster.advance;
        if advance <= 0.0f64 {
            return;
        }
        auto_space_edge_trims[usize::try_from(cluster_idx).unwrap_or(0)] += advance;
        auto_space_edge_decisions.push(LineEdgeTrimDecisionInfo::new((line_source_range).clone(), (cluster.range).clone(), side, advance, 0.0f64, advance, "LineEdgeWordSpaceCollapse"));
    }

    pub fn line_adjustment_stage_finish_paragraph_layout(engine: &mut ExplainableStubParagraphLayoutEngine, prep: ParagraphLayoutPrep, plan: LineBreakPlanningStageResult) -> Result<LayoutResult, LineAdjustmentStageFinishParagraphLayoutFault> {
        let mut applied_hanging_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        {
            let _g1 = ((plan.line_solution).clone().lines).clone();
            for line in &_g1 {
                for i in 0..u32::from_ne_bytes(((line.hanging_cluster_indices).clone().size()).to_ne_bytes()) {
                    applied_hanging_clusters_builder.put(&((line.hanging_cluster_indices).clone().at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
                }
            }
        }
        let applied_hanging_clusters: SortedSetTable<u32> = applied_hanging_clusters_builder.clone().build();
        let mut impossible_measure_contextual_hang_clusters_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes(((plan.ascii_point_mark_kinsoku).clone().impossible_measure_hang_eligible_clusters.size()).to_ne_bytes()) {
            impossible_measure_contextual_hang_clusters_builder.put(&((plan.ascii_point_mark_kinsoku).clone().impossible_measure_hang_eligible_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        for i in 0..u32::from_ne_bytes(((plan.inline_object_kinsoku).clone().impossible_measure_hang_eligible_clusters.size()).to_ne_bytes()) {
            impossible_measure_contextual_hang_clusters_builder.put(&((plan.inline_object_kinsoku).clone().impossible_measure_hang_eligible_clusters.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        let impossible_measure_contextual_hang_clusters: SortedSetTable<u32> = impossible_measure_contextual_hang_clusters_builder.clone().build();
        let mut raw_decisions: Vec<ContextualKinsokuDecisionInfo> = Vec::new();
        {
            let _g1 = ((plan.ascii_point_mark_kinsoku).clone().decisions).clone();
            for d in &_g1 {
                raw_decisions.push(d.clone());
            }
        }
        {
            let _g1 = ((plan.inline_object_kinsoku).clone().decisions).clone();
            for d in &_g1 {
                raw_decisions.push(d.clone());
            }
        }
        {
            let _g1 = ((plan.unicode_punctuation_boundaries).clone().decisions).clone();
            for d in &_g1 {
                raw_decisions.push(d.clone());
            }
        }
        let mut seen_decision_keys: Vec<String> = Vec::new();
        let mut contextual_kinsoku_decisions: Vec<ContextualKinsokuDecisionInfo> = Vec::new();
        for decision in &raw_decisions {
            let key = format!("{}{}{}",
            (decision.range).clone().to_string(),
            ":",
            (decision.forbidden_position).to_string()
        );
            let mut already_seen = false;
            {
                let mut _g = 0u32;
                while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((seen_decision_keys.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let s = (seen_decision_keys[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    if s == key {
                        already_seen = true;
                        break;
                    }
                }
            }
            if !already_seen {
                seen_decision_keys.push(key.clone());
                let c_idx = decision.cluster_index;
                if impossible_measure_contextual_hang_clusters.has(&(c_idx)) && applied_hanging_clusters.has(&(c_idx)) {
                    let fallback = if decision.reason.to_string() == "AttachedAsciiPointMarkKinsoku" { "AttachedAsciiPointMarkImpossibleMeasureHang".to_string() } else { "InlineObjectAttachedMarkImpossibleMeasureHang".to_string() };
                    contextual_kinsoku_decisions.push(ContextualKinsokuDecisionInfo::new((decision.range).clone(), (decision.source_text).to_string().as_str(), decision.cluster_index, (decision.forbidden_position).to_string().as_str(), (decision.reason).to_string().as_str(),
Some((fallback).to_string())));
                } else {
                    contextual_kinsoku_decisions.push(decision.clone());
                }
            }
        }
        let mut push_in_trailing: Vec<f64> = Vec::new();
        let mut push_in_leading: Vec<f64> = Vec::new();
        let mut push_in_raw_trims: Vec<f64> = Vec::new();
        for _ in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            push_in_trailing.push(0.0f64);
            push_in_leading.push(0.0f64);
            push_in_raw_trims.push(0.0f64);
        }
        for l_idx in 0..match u32::try_from((plan.line_solution).clone().lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = ((plan.line_solution).clone().lines[usize::try_from(l_idx).unwrap_or(0)]).clone();
            let repair = line.repair.clone();
            match &(repair) {
                Some(__option19) => {
                    let allocations = RepairOptions::repair_options_push_in_allocations((*__option19).clone());
                    match &(allocations) {
                        Some(__option20) => {
                            for alloc in __option20 {
                                if alloc.cluster_index <= 2147483647 && (i32::from_ne_bytes((alloc.cluster_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((prep.natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                                    if alloc.channel == ShrinkChannel::TrailingGlue {
                                        let index = alloc.cluster_index;
                                        push_in_trailing[usize::try_from(index).unwrap_or(0)] += alloc.shrink;
                                    } else {
                                        if alloc.channel == ShrinkChannel::LeadingGlue {
                                            let index = alloc.cluster_index;
                                            push_in_leading[usize::try_from(index).unwrap_or(0)] += alloc.shrink;
                                        } else {
                                            if alloc.channel == ShrinkChannel::LeadingAndTrailingGlue {
                                                let index = alloc.cluster_index;
                                                push_in_leading[usize::try_from(index).unwrap_or(0)] += alloc.shrink / 2.0f64;
                                                let index = alloc.cluster_index;
                                                push_in_trailing[usize::try_from(index).unwrap_or(0)] += alloc.shrink / 2.0f64;
                                            } else {
                                                if alloc.channel == ShrinkChannel::RawAdvance {
                                                    let index = alloc.cluster_index;
                                                    push_in_raw_trims[usize::try_from(index).unwrap_or(0)] += alloc.shrink;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                        }
                    }
                }
                None => {
                }
            }
        }
        if i32::from_ne_bytes((u32::from_ne_bytes((prep.hyphen_offsets.size()).to_ne_bytes())).to_ne_bytes()) > (0) {
            for line_index in 0..match u32::try_from((plan.line_solution).clone().lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let line = ((plan.line_solution).clone().lines[usize::try_from(line_index).unwrap_or(0)]).clone();
                if line.cluster_range.get_is_empty() {
                    continue;
                }
                let hyphen = LineAdjustmentStage::line_adjustment_stage_line_hyphen_advance_at(line_index, &(plan.line_solution).clone().lines, (prep.hyphen_offsets).clone(), &prep.natural_clusters, prep.hyphen_advance);
                if hyphen <= 0.0f64 {
                    continue;
                }
                let line_limit = if line.cluster_range.start == 0 { prep.measure - plan.first_line_indent } else { prep.measure - plan.block_indent };
                let mut content_terms: Vec<f64> = vec![];
                for c_idx in line.cluster_range.start..u32::wrapping_add(line.cluster_range.end, 1) {
                    content_terms.push(prep.clusters[usize::try_from(c_idx).unwrap_or(0)].advance);
                }
                let content = AccurateSum::accurate_sum_of(&content_terms);
                let mut shortfall = content + hyphen - line_limit;
                if shortfall <= 0.001f64 {
                    continue;
                }
                let mut sorted_opportunities: Vec<ShrinkOpportunity> = Vec::new();
                for opp_idx in 0..match u32::try_from(prep.shrink_opportunities.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let opp = (prep.shrink_opportunities[usize::try_from(opp_idx).unwrap_or(0)]).clone();
                    if i32::from_ne_bytes((opp.cluster_index).to_ne_bytes()) >= i32::from_ne_bytes((line.cluster_range.start).to_ne_bytes()) && (i32::from_ne_bytes((opp.cluster_index).to_ne_bytes())) <= i32::from_ne_bytes((line.cluster_range.end).to_ne_bytes()) &&
!opp.line_end_only {
                        sorted_opportunities.push(opp.clone());
                    }
                }
                let mut r_idx = 1u32;
                while (i32::from_ne_bytes((r_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((sorted_opportunities.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let curr = (sorted_opportunities[usize::try_from(r_idx).unwrap_or(0)]).clone();
                    let mut j = r_idx;
                    while (j) > (0) && (i32::from_ne_bytes((sorted_opportunities[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)].tier).to_ne_bytes())) > (i32::from_ne_bytes((curr.tier).to_ne_bytes())) {
                        sorted_opportunities[usize::try_from(j).unwrap_or(0)] = (sorted_opportunities[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]).clone();
                        j = u32::wrapping_sub(j, 1);
                    }
                    sorted_opportunities[usize::try_from(j).unwrap_or(0)] = curr;
                    r_idx = u32::wrapping_add(r_idx, 1);
                }
                for opp in &sorted_opportunities {
                    if shortfall <= 0.001f64 {
                        break;
                    }
                    let mut used = 0.0f64;
                    if opp.channel == ShrinkChannel::TrailingGlue {
                        used = push_in_trailing[usize::try_from(opp.cluster_index).unwrap_or(0)];
                    } else {
                        if opp.channel == ShrinkChannel::LeadingGlue {
                            used = push_in_leading[usize::try_from(opp.cluster_index).unwrap_or(0)];
                        } else {
                            if opp.channel == ShrinkChannel::RawAdvance {
                                used = push_in_raw_trims[usize::try_from(opp.cluster_index).unwrap_or(0)];
                            } else {
                                if opp.channel == ShrinkChannel::LeadingAndTrailingGlue {
                                    used = push_in_leading[usize::try_from(opp.cluster_index).unwrap_or(0)] + push_in_trailing[usize::try_from(opp.cluster_index).unwrap_or(0)];
                                }
                            }
                        }
                    }
                    let avail = opp.capacity - used;
                    let target = if shortfall < (avail) { shortfall } else { avail };
                    let take = if target < (0.0f64) { 0.0f64 } else { target };
                    if take <= 0.0f64 {
                        continue;
                    }
                    if opp.channel == ShrinkChannel::TrailingGlue {
                        let index = opp.cluster_index;
                        push_in_trailing[usize::try_from(index).unwrap_or(0)] += take;
                    } else {
                        if opp.channel == ShrinkChannel::LeadingGlue {
                            let index = opp.cluster_index;
                            push_in_leading[usize::try_from(index).unwrap_or(0)] += take;
                        } else {
                            if opp.channel == ShrinkChannel::LeadingAndTrailingGlue {
                                let index = opp.cluster_index;
                                push_in_leading[usize::try_from(index).unwrap_or(0)] += take / 2.0f64;
                                let index = opp.cluster_index;
                                push_in_trailing[usize::try_from(index).unwrap_or(0)] += take / 2.0f64;
                            } else {
                                if opp.channel == ShrinkChannel::RawAdvance {
                                    let index = opp.cluster_index;
                                    push_in_raw_trims[usize::try_from(index).unwrap_or(0)] += take;
                                }
                            }
                        }
                    }
                    shortfall -= take;
                }
            }
        }
        let mut push_in_trailing_map_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut push_in_leading_map_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if push_in_trailing[usize::try_from(i).unwrap_or(0)] != 0.0f64 {
                push_in_trailing_map_builder.put(&(i), &(push_in_trailing[usize::try_from(i).unwrap_or(0)]));
            }
            if push_in_leading[usize::try_from(i).unwrap_or(0)] != 0.0f64 {
                push_in_leading_map_builder.put(&(i), &(push_in_leading[usize::try_from(i).unwrap_or(0)]));
            }
        }
        let push_in_geometry = (prep.base_geometry).clone().consume_trailing_by_cluster(push_in_trailing_map_builder.clone().build()).consume_leading_by_cluster(push_in_leading_map_builder.clone().build());
        let edge_trim_result = push_in_geometry.consume_line_edge_glue(&(plan.line_solution).clone().lines, Some((prep.adjustment_style).clone().line_end_punctuation == LineEndPunctuationStyle::ForceHalfWidth));
        let auto_space_gap = ((prep.clreq_profile).clone().auto_space).clone().gap_em * prep.font_size;
        let capacity = prep.natural_clusters.len();
        let mut auto_space_edge_trims = Vec::with_capacity(capacity);
        for _ in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            auto_space_edge_trims.push(0.0f64);
        }
        let mut auto_space_edge_decisions: Vec<LineEdgeTrimDecisionInfo> = Vec::new();
        {
            let _g1 = ((plan.line_solution).clone().lines).clone();
            for line in &_g1 {
                if line.cluster_range.clone().get_is_empty() {
                    continue;
                }
                LineAdjustmentStage::line_adjustment_stage_trim_edge((line.source_range).clone(), (line.cluster_range).clone().end, &"trailing", &prep.natural_clusters, &prep.auto_space_decisions, auto_space_gap, &mut auto_space_edge_trims, &mut auto_space_edge_decisions);
                LineAdjustmentStage::line_adjustment_stage_trim_edge((line.source_range).clone(), (line.cluster_range).clone().start, &"leading", &prep.natural_clusters, &prep.auto_space_decisions, auto_space_gap, &mut auto_space_edge_trims, &mut auto_space_edge_decisions);
                LineAdjustmentStage::line_adjustment_stage_collapse_edge_space((line.source_range).clone(), (line.cluster_range).clone().end, &"trailing", &prep.natural_clusters, (prep.inline_object_separator_space_trims).clone(), &mut auto_space_edge_trims, &mut
auto_space_edge_decisions);
                LineAdjustmentStage::line_adjustment_stage_collapse_edge_space((line.source_range).clone(), (line.cluster_range).clone().start, &"leading", &prep.natural_clusters, (prep.inline_object_separator_space_trims).clone(), &mut auto_space_edge_trims, &mut
auto_space_edge_decisions);
                let attached_glue_cluster = (line.cluster_range).clone().end;
                let attached_glue = if prep.attached_punctuation_trailing_glue_by_cluster.has(&(attached_glue_cluster)) { (prep.attached_punctuation_trailing_glue_by_cluster.get(&(attached_glue_cluster))).unwrap() } else { 0.0f64 };
                if attached_glue > (0.0f64) {
                    auto_space_edge_trims[usize::try_from(attached_glue_cluster).unwrap_or(0)] += attached_glue;
                    auto_space_edge_decisions.push(LineEdgeTrimDecisionInfo::new((line.source_range).clone(), ((prep.natural_clusters[usize::try_from(attached_glue_cluster).unwrap_or(0)]).clone().range).clone(), "trailing", attached_glue, 0.0f64, attached_glue,
"AttachedInlineVirtualBoundaryLineEndTrim"));
                }
                if line.end_reason == LineEndReason::AutoWrap {
                    let cluster_idx = (line.cluster_range).clone().end;
                    let discardable = if prep.inline_object_by_cluster_index.has(&(cluster_idx)) { (prep.inline_object_by_cluster_index.get(&(cluster_idx)).as_ref().unwrap().trailing_boundary).clone().line_end_discardable_advance } else { 0.0f64 };
                    let consumed_before = if push_in_raw_trims[usize::try_from(cluster_idx).unwrap_or(0)] < (discardable) { push_in_raw_trims[usize::try_from(cluster_idx).unwrap_or(0)] } else { discardable };
                    let diff = discardable - consumed_before;
                    let remaining = if diff < (0.0f64) { 0.0f64 } else { diff };
                    if remaining > (0.0f64) {
                        auto_space_edge_trims[usize::try_from(cluster_idx).unwrap_or(0)] += remaining;
                        auto_space_edge_decisions.push(LineEdgeTrimDecisionInfo::new((line.source_range).clone(), ((prep.natural_clusters[usize::try_from(cluster_idx).unwrap_or(0)]).clone().range).clone(), "trailing", remaining, consumed_before, discardable,
"InlineObjectLineEndDiscardableGlue"));
                    }
                }
            }
        }
        let mut raw_trims_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let total = auto_space_edge_trims[usize::try_from(i).unwrap_or(0)] + push_in_raw_trims[usize::try_from(i).unwrap_or(0)];
            if total != 0.0f64 {
                raw_trims_builder.put(&(i), &(total));
            }
        }
        let trimmed_geometry = (edge_trim_result.geometry).clone().with_raw_edge_trims(raw_trims_builder.clone().build());
        let trimmed_clusters = trimmed_geometry.resolve_clusters();
        let mut edge_trim_decisions: Vec<LineEdgeTrimDecisionInfo> = Vec::new();
        {
            let _g1 = edge_trim_result.decisions.clone();
            for d in &_g1 {
                edge_trim_decisions.push(d.clone());
            }
        }
        for d in &auto_space_edge_decisions {
            edge_trim_decisions.push(d.clone());
        }
        let mut justification_plans: Vec<Option<JustificationPlan>> = Vec::new();
        for line_index in 0..match u32::try_from((plan.line_solution).clone().lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line_candidate = ((plan.line_solution).clone().lines[usize::try_from(line_index).unwrap_or(0)]).clone();
            let is_last = line_index == u32::wrapping_sub(u32::try_from(((plan.line_solution).clone().lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
            if is_last || line_candidate.cluster_range.get_is_empty() || line_candidate.end_reason != LineEndReason::AutoWrap {
                justification_plans.push(None);
            } else {
                let next_idx = u32::wrapping_add(line_candidate.cluster_range.end, 1);
                let selected_technical_break = if plan.progressive_break_opportunities.has(&(next_idx)) { plan.progressive_break_opportunities.get(&(next_idx)) } else { None };
                let preferred_tracking_span = match &(selected_technical_break) { Some(__option22) => (if __option22.tier == ProgressiveBreakTier::Emergency { Some((__option22.span_range).clone()) } else { None }).clone(), None => None };
                let mut preferred_emergency_tracking_boundaries_builder: SortedMapTableBuilder<u32, String> = SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
                match &(preferred_tracking_span) {
                    Some(__option23) => {
                        for i in 0..u32::from_ne_bytes((plan.emergency_tracking_boundary_after_clusters.size()).to_ne_bytes()) {
                            let left_index = plan.emergency_tracking_boundary_after_clusters.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
                            let reason = plan.emergency_tracking_boundary_after_clusters.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
                            let right_index = u32::wrapping_add(left_index, 1);
                            if i32::from_ne_bytes((((prep.natural_clusters[usize::try_from(left_index).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()) >= i32::from_ne_bytes((__option23.start).to_ne_bytes()) &&
(i32::from_ne_bytes((((prep.natural_clusters[usize::try_from(right_index).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes((__option23.end).to_ne_bytes()) {
                                preferred_emergency_tracking_boundaries_builder.put(&(left_index), &(reason).to_string());
                            }
                        }
                    }
                    None => {
                    }
                }
                let preferred_emergency_tracking_boundaries: SortedMapTable<u32, String> = preferred_emergency_tracking_boundaries_builder.clone().build();
                let hyphen_advance_for_line = LineAdjustmentStage::line_adjustment_stage_line_hyphen_advance_at(line_index, &(plan.line_solution).clone().lines, (prep.hyphen_offsets).clone(), &prep.natural_clusters, prep.hyphen_advance);
                let line_limit = (if line_candidate.cluster_range.start == 0 { prep.measure - plan.first_line_indent } else { prep.measure - plan.block_indent }) - hyphen_advance_for_line;
                let plan_result = engine.justifier.justify(&trimmed_clusters, &prep.cluster_roles, &prep.east_asian_spacing_edges, line_candidate.get_in_measure_cluster_range(), line_limit, prep.font_size, false, None.clone(),
Some((prep.adjustment_style).clone().allow_sino_western_gap_adjustment), ((prep.clreq_profile).clone().auto_space).clone().gap_em, ((prep.clreq_profile).clone().auto_space).clone().stretch_max_em, Some((plan.no_stretch_boundary_clusters).clone()),
Some((plan.no_stretch_boundary_after_clusters).clone()), Some((plan.western_bracket_cjk_inter_char_boundary_after_clusters).clone()), Some((plan.attached_inline_physical_boundary_after_clusters).clone()), Some((plan.attached_inline_virtual_boundary_after_clusters).clone()),
Some((plan.attached_inline_virtual_sino_western_boundary_after_clusters).clone()), Some((prep.uniform_inline_object_boundary_after_clusters).clone()), Some((prep.preferred_inline_object_boundary_after_clusters).clone()), Some((plan.technical_boundary_after_clusters).clone()),
Some((plan.emergency_tracking_boundary_after_clusters).clone()), Some((preferred_emergency_tracking_boundaries).clone())).map_err(|e| LineAdjustmentStageFinishParagraphLayoutFault::TextRangeErrorFault(e))?;
                justification_plans.push(Some(plan_result));
            }
        }
        let current_line_technical_body_stretch_limit = 0.0f64 * prep.font_size;
        let mut newly_rejected_spans: Vec<TextRange> = Vec::new();
        let mut newly_rejected_tiers_list: Vec<SortedSetTable<u32>> = Vec::new();
        for line_index in 0..match u32::try_from((plan.line_solution).clone().lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = ((plan.line_solution).clone().lines[usize::try_from(line_index).unwrap_or(0)]).clone();
            if line.end_reason != LineEndReason::AutoWrap || (i32::from_ne_bytes((line.cluster_range.start).to_ne_bytes())) > (i32::from_ne_bytes((line.cluster_range.end).to_ne_bytes())) {
                continue;
            }
            let next_cluster = u32::wrapping_add(line.cluster_range.end, 1);
            if !plan.progressive_break_opportunities.has(&(next_cluster)) {
                continue;
            }
            let selected_technical_break = plan.progressive_break_opportunities.get(&(next_cluster));
            if selected_technical_break.as_ref().unwrap().tier == ProgressiveBreakTier::Emergency {
                continue;
            }
            let span_range = (selected_technical_break.as_ref().unwrap().span_range).clone().clone();
            if prep.rejected_technical_tiers_by_span.has(&(span_range)) {
                let rejected_for_span = (prep.rejected_technical_tiers_by_span.get(&(span_range))).unwrap();
                if rejected_for_span.has(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(selected_technical_break.as_ref().unwrap().tier))) {
                    continue;
                }
            }
            let current_line_plan = (justification_plans[usize::try_from(line_index).unwrap_or(0)]).clone();
            if current_line_plan.is_none() {
                continue;
            }
            let mut current_line_uses_unbounded_tracking = false;
            {
                let _g1 = ((current_line_plan).as_ref().unwrap().allocations).clone().clone();
                for alloc in &_g1 {
                    if (alloc.kind == GlueKind::CjkInterChar || alloc.kind == GlueKind::EmergencyGraphemeTracking) && (alloc.delta) > (current_line_technical_body_stretch_limit + 0.001f64) {
                        current_line_uses_unbounded_tracking = true;
                        break;
                    }
                }
            }
            if current_line_uses_unbounded_tracking {
                let mut found_span_idx = 4294967295u32;
                for s_idx in 0..match u32::try_from(newly_rejected_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if newly_rejected_spans[usize::try_from(s_idx).unwrap_or(0)].start == span_range.start && newly_rejected_spans[usize::try_from(s_idx).unwrap_or(0)].end == span_range.end {
                        found_span_idx = s_idx;
                        break;
                    }
                }
                if found_span_idx > 2147483647 {
                    newly_rejected_spans.push(span_range.clone());
                    let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
                    b.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(selected_technical_break.as_ref().unwrap().tier)));
                    newly_rejected_tiers_list.push(b.clone().build());
                } else {
                    let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
                    let existing: SortedSetTable<u32> = (newly_rejected_tiers_list[usize::try_from(found_span_idx).unwrap_or(0)]).clone();
                    for e in 0..u32::from_ne_bytes((existing.size()).to_ne_bytes()) {
                        b.put(&(existing.at({ let v: u32 = e; i32::from_ne_bytes(v.to_ne_bytes()) })));
                    }
                    b.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(selected_technical_break.as_ref().unwrap().tier)));
                    newly_rejected_tiers_list[usize::try_from(found_span_idx).unwrap_or(0)] = b.clone().build();
                }
            }
        }
        if i32::from_ne_bytes((u32::try_from((newly_rejected_spans.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            let mut updated_rejected_tiers_builder: SortedMapTableBuilder<TextRange, SortedSetTable<u32>> = SortedTable::sorted_table_map_builder::<TextRange, SortedSetTable<u32>>(Arc::new(compare_text_range));
            for i in 0..u32::from_ne_bytes((prep.rejected_technical_tiers_by_span.size()).to_ne_bytes()) {
                let span = prep.rejected_technical_tiers_by_span.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
                let tiers: SortedSetTable<u32> = prep.rejected_technical_tiers_by_span.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
                let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
                for t in 0..u32::from_ne_bytes((tiers.size()).to_ne_bytes()) {
                    b.put(&(tiers.at({ let v: u32 = t; i32::from_ne_bytes(v.to_ne_bytes()) })));
                }
                for n_idx in 0..match u32::try_from(newly_rejected_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if newly_rejected_spans[usize::try_from(n_idx).unwrap_or(0)].start == span.start && newly_rejected_spans[usize::try_from(n_idx).unwrap_or(0)].end == span.end {
                        let add_tiers: SortedSetTable<u32> = (newly_rejected_tiers_list[usize::try_from(n_idx).unwrap_or(0)]).clone();
                        for at in 0..u32::from_ne_bytes((add_tiers.size()).to_ne_bytes()) {
                            b.put(&(add_tiers.at({ let v: u32 = at; i32::from_ne_bytes(v.to_ne_bytes()) })));
                        }
                    }
                }
                updated_rejected_tiers_builder.put(&(span), &(b.clone().build()));
            }
            for n_idx in 0..match u32::try_from(newly_rejected_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let span = (newly_rejected_spans[usize::try_from(n_idx).unwrap_or(0)]).clone();
                if !prep.rejected_technical_tiers_by_span.has(&(span)) {
                    updated_rejected_tiers_builder.put(&(span), &((newly_rejected_tiers_list[usize::try_from(n_idx).unwrap_or(0)]).clone()));
                }
            }
            return Ok(engine.layout_with_rejected_technical_tiers((prep.input).clone(), updated_rejected_tiers_builder.clone().build())?);
        }
        let capacity = prep.natural_clusters.len();
        let mut justify_deltas = Vec::with_capacity(capacity);
        for _ in 0..match u32::try_from(prep.natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            justify_deltas.push(0.0f64);
        }
        for p in &justification_plans {
            match &(p) {
                Some(__option24) => {
                    let mut _g = 0u32;
                    let _g1 = (__option24.allocations).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let alloc = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if alloc.target_cluster_index <= 2147483647 && (i32::from_ne_bytes((alloc.target_cluster_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((justify_deltas.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                            let index = alloc.target_cluster_index;
                            justify_deltas[usize::try_from(index).unwrap_or(0)] += alloc.delta;
                        }
                    }
                }
                None => {
                }
            }
        }
        let mut justify_delta_builder: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(justify_deltas.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if justify_deltas[usize::try_from(i).unwrap_or(0)] != 0.0f64 {
                justify_delta_builder.put(&(i), &(justify_deltas[usize::try_from(i).unwrap_or(0)]));
            }
        }
        let justify_delta_by_cluster: SortedMapTable<u32, f64> = justify_delta_builder.clone().build();
        let final_geometry = trimmed_geometry.add_justification_deltas((justify_delta_by_cluster).clone());
        let resolved_final_clusters = final_geometry.resolve_clusters();
        let mut final_clusters: Vec<Cluster> = Vec::new();
        for c in &resolved_final_clusters {
            if !plan.metric_decision_by_range.has(&((c.range).clone())) {
                final_clusters.push(c.clone());
            } else {
                let m = (plan.metric_decision_by_range.get(&((c.range).clone())).as_ref().unwrap().layout_metrics).clone();
                let metric_shift = if m.baseline_class == BaselineClass::Roman { 0.0f64 } else { plan.base_box_descent - m.descent };
                let shift = c.baseline_shift + metric_shift + (prep.style_at)((c.range).clone().start).baseline_shift;
                if shift > (-0.01f64) && (shift) < (0.01f64) {
                    final_clusters.push(c.clone());
                } else {
                    final_clusters.push(Cluster::new((c.range).clone(), (c.text).to_string().as_str(), (c.font_key).to_string().as_str(), c.advance, Some((c.display_text).to_string()), Some(shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift)));
                }
            }
        }
        let geometry_decisions = final_geometry.to_decision_info();
        let run_groups = LineAdjustmentStage::line_adjustment_stage_renderable_glyph_run_clusters(&final_clusters, (prep.open_type_features_by_cluster_range).clone());
        let capacity = run_groups.len();
        let mut glyph_runs = Vec::with_capacity(capacity);
        for _g_index11 in 0..match u32::try_from(run_groups.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run_clusters = (run_groups[usize::try_from(_g_index11).unwrap_or(0)]).clone();
            let first_c = (run_clusters[0usize]).clone();
            let last_c = (run_clusters[usize::try_from(u32::wrapping_sub(u32::try_from((run_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
            let open_type_features = if prep.open_type_features_by_cluster_range.has(&((first_c.range).clone())) { (prep.open_type_features_by_cluster_range.get(&((first_c.range).clone()))).unwrap() } else { vec![] };
            let mut run_glyphs: Vec<Glyph> = Vec::new();
            let mut run_advance_terms: Vec<f64> = vec![];
            for cluster_idx in 0..match u32::try_from(run_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let cluster = (run_clusters[usize::try_from(cluster_idx).unwrap_or(0)]).clone();
                run_advance_terms.push(cluster.advance);
                if prep.shaped_glyphs_by_cluster_range.has(&((cluster.range).clone())) {
                    let shaped = (prep.shaped_glyphs_by_cluster_range.get(&((cluster.range).clone()))).unwrap();
                    let mapped = ParagraphShapingStage::paragraph_shaping_stage_map_to_cluster_range(&shaped, (cluster).clone());
                    let centered = LineAdjustmentStage::line_adjustment_stage_center_dash_ink(&mapped, (cluster).clone(), (prep.atom_class_by_range).clone());
                    {
                        let mut _g = 0u32;
                        while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((centered.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                            let g = (centered[usize::try_from(_g).unwrap_or(0)]).clone();
                            _g = u32::wrapping_add(_g, 1);
                            run_glyphs.push(g.clone());
                        }
                    }
                } else {
                    run_glyphs.push(Glyph::new(cluster_idx, (cluster.range).clone(), cluster.advance, Some(0.0), Some(0.0), None, None, None, None));
                }
            }
            glyph_runs.push(GlyphRun::new(TextRange::new((first_c.range).clone().start, (last_c.range).clone().end).map_err(|e| LineAdjustmentStageFinishParagraphLayoutFault::TextRangeErrorFault(e))?, (first_c.font_key).to_string().as_str(), run_glyphs.to_vec(),
AccurateSum::accurate_sum_of(&run_advance_terms), Some((open_type_features).clone())));
        }
        let vertical_geometry = LineGeometryStageFns::line_geometry_stage_fns_resolve_line_vertical_geometry_sorted((prep.input).clone(), prep.font_size, &prep.pinyin_spans, &prep.natural_clusters, (plan.line_solution).clone(), (prep.ruby_font_geometry_by_span).clone(),
plan.existing_interline_space, (plan.base_line_metrics).clone(), plan.base_face_height, plan.ruby_extent, Some((prep.inline_object_by_cluster_index).clone()), plan.base_ascent, plan.base_descent);
        let ruby_line_height_decision = vertical_geometry.ruby_line_height_decision.clone();
        let inline_object_line_height_decision = vertical_geometry.inline_object_line_height_decision.clone();
        let line_baseline = vertical_geometry.line_baseline.clone();
        let line_top = vertical_geometry.line_top.clone();
        let line_bottom = vertical_geometry.line_bottom.clone();
        let line_boxes = LineAdjustmentStage::line_adjustment_stage_build_line_boxes((prep.input).clone(), (plan.line_solution).clone(), &trimmed_clusters, &final_clusters, plan.first_line_indent, plan.block_indent, prep.measure, prep.grid_body_offset, &line_baseline, &line_top,
&line_bottom, (prep.hyphen_offsets).clone(), &prep.natural_clusters, prep.hyphen_advance, &prep.hyphen_glyphs, &justification_plans);
        let laid_out_lines = line_boxes.laid_out_lines.clone();
        let lines = line_boxes.visible_lines.clone();
        let max_lines_decision = line_boxes.max_lines_decision.clone();
        let visible_line_ranges = line_boxes.visible_line_ranges.clone();
        let annotation_geometry = LineAdjustmentStage::line_adjustment_stage_resolve_annotation_geometry(engine, (prep.input).clone(), prep.font_size, (prep.inline_object_by_cluster_index).clone(), (plan.line_solution).clone(), (prep.clreq_profile).clone(), &geometry_decisions,
&prep.auto_space_decisions, &visible_line_ranges, &lines, &final_clusters, &prep.cluster_roles, (justify_delta_by_cluster).clone(), (prep.ruby_and_bopomofo_spread).clone(), &plan.metric_decisions, &prep.pinyin_spans, &prep.natural_clusters,
(prep.ruby_font_geometry_by_span).clone(), prep.ruby_stack_gap, plan.base_ascent, prep.ruby_font_size, prep.ruby_font_weight, plan.base_descent, (prep.bopomofo_font_weight_at).clone()).map_err(|e| LineAdjustmentStageFinishParagraphLayoutFault::TextShaperShapeFaultFault(e))?;
        let inline_object_decisions = annotation_geometry.inline_object_decisions.clone();
        let decoration_decisions = annotation_geometry.decoration_decisions.clone();
        let decoration_segments = annotation_geometry.decoration_segments.clone();
        let ruby_decisions = annotation_geometry.ruby_decisions.clone();
        let bopomofo_decisions = annotation_geometry.bopomofo_decisions.clone();
        let mut widest_line = 0.0f64;
        for line in &lines {
            let w = line.indent + line.visual_width + line.hyphen_advance;
            if w > (widest_line) {
                widest_line = w;
            }
        }
        let total_height: f64;
        if i32::from_ne_bytes((u32::try_from((lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            total_height = lines[usize::try_from(u32::wrapping_sub(u32::try_from((lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].bottom;
        } else {
            if u_string::unit_count(&((prep.text).to_string())) == 0 {
                total_height = 0.0f64;
            } else {
                total_height = (plan.base_line_metrics).clone().height;
            }
        }
        let max_width_constraint = ((prep.input).clone().constraints).clone().max_width;
        let result_width = if widest_line > (max_width_constraint) { max_width_constraint } else { widest_line };
        return Ok(LayoutResult::new((prep.input).clone(), Size::new(result_width, total_height), (final_clusters).clone(), (glyph_runs).clone(), (lines).clone(), LayoutDebugAssembly::layout_debug_assembly_build_layout_debug_info((engine).clone(),
LayoutDebugStageInput::new((prep.text).to_string().as_str(), prep.font_decisions.to_vec(), (prep.punctuation_glyph_substitutor).clone(), (prep.substitution_rollbacks).clone(), prep.shaping_decisions.to_vec(), plan.metric_decisions.to_vec(), prep.punctuation_atoms.to_vec(),
geometry_decisions.to_vec(), (prep.spacing_plan).clone(), (prep.attached_punctuation_boundary).clone(), prep.role_override_infos.to_vec(), laid_out_lines.to_vec(), (plan.line_solution).clone(), prep.clusters.to_vec(), justification_plans.to_vec(),
prep.auto_space_decisions.to_vec(), edge_trim_decisions.to_vec(), decoration_decisions.to_vec(), decoration_segments.to_vec(), ruby_decisions.to_vec(), bopomofo_decisions.to_vec(), prep.mandatory_break_decisions.to_vec(), (max_lines_decision).clone(),
(plan.line_spacing_decision).clone(), (ruby_line_height_decision).clone(), (inline_object_line_height_decision).clone(), (plan.kinsoku_decision).clone(), contextual_kinsoku_decisions.to_vec(), (prep.line_length_grid_decision).clone(), (plan.first_line_indent_decision).clone(),
(prep.inline_box_result).clone().decisions.to_vec(), inline_object_decisions.to_vec(), prep.inline_object_punctuation_attachment_decisions.to_vec(), prep.zero_width_break_decisions.to_vec(), prep.break_opportunity_decisions.to_vec(),
prep.emergency_tracking_eligibility_decisions.to_vec(), (plan.progressive_break_opportunities).clone())).map_err(|e| LineAdjustmentStageFinishParagraphLayoutFault::UStringFaultFault(e))?));
    }
}
