use crate::org::tiqian::clreq::auto_space_mode::AutoSpaceMode;
use crate::org::tiqian::clreq::auto_space_policy::AutoSpacePolicy;
use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::contextual_kinsoku_decision_info::ContextualKinsokuDecisionInfo;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_decision_info::InlineBoxDecisionInfo;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::kinsoku_rule::KinsokuRule;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationInkInput;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaryResolver;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Clone, PartialEq)]
pub struct ContextualKinsoku {
    pub forbidden_line_start_clusters: SortedSetTable<u32>,
    pub unbreakable_ranges: Vec<IntRange>,
    pub impossible_measure_hang_eligible_clusters: SortedSetTable<u32>,
    pub extendable_hang_ranges: Vec<IntRange>,
    pub decisions: Vec<ContextualKinsokuDecisionInfo>,
}

impl ContextualKinsoku {
    pub fn new(forbidden_line_start_clusters: SortedSetTable<u32>, unbreakable_ranges: Vec<IntRange>, impossible_measure_hang_eligible_clusters: SortedSetTable<u32>, extendable_hang_ranges: Vec<IntRange>, decisions: Vec<ContextualKinsokuDecisionInfo>) -> Self {
        Self {
            forbidden_line_start_clusters,
            unbreakable_ranges,
            impossible_measure_hang_eligible_clusters,
            extendable_hang_ranges,
            decisions,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ContextualKinsoku(",
            "forbiddenLineStartClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.forbidden_line_start_clusters).clone();
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
            "unbreakableRanges=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.unbreakable_ranges).clone();
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
            "impossibleMeasureHangEligibleClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.impossible_measure_hang_eligible_clusters).clone();
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
            "extendableHangRanges=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.extendable_hang_ranges).clone();
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
            "decisions=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decisions).clone();
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

#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectAttachedMark {
    pub object_cluster_index: u32,
    pub separator_cluster_indices: Vec<u32>,
    pub mark_cluster_index: u32,
}

impl InlineObjectAttachedMark {
    pub fn new(object_cluster_index: u32, separator_cluster_indices: Vec<u32>, mark_cluster_index: u32) -> Self {
        Self {
            object_cluster_index,
            separator_cluster_indices,
            mark_cluster_index,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "InlineObjectAttachedMark(",
            "objectClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.object_cluster_index),
            ", ",
            "separatorClusterIndices=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.separator_cluster_indices).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(arr[i]));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "markClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.mark_cluster_index),
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AutoSpaceApplicationResult {
    pub clusters: Vec<Cluster>,
    pub decisions: Vec<AutoSpaceDecisionInfo>,
}

impl AutoSpaceApplicationResult {
    pub fn new(clusters: Vec<Cluster>, decisions: Vec<AutoSpaceDecisionInfo>) -> Self {
        Self {
            clusters,
            decisions,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "AutoSpaceApplicationResult(",
            "clusters=",
            {
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
    },
            ", ",
            "decisions=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decisions).clone();
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

#[derive(Clone, PartialEq)]
pub struct InlineBoxApplicationResult {
    pub clusters: Vec<Cluster>,
    pub advance_by_cluster: SortedMapTable<u32, f64>,
    pub decisions: Vec<InlineBoxDecisionInfo>,
}

impl InlineBoxApplicationResult {
    pub fn new(clusters: Vec<Cluster>, advance_by_cluster: SortedMapTable<u32, f64>, decisions: Vec<InlineBoxDecisionInfo>) -> Self {
        Self {
            clusters,
            advance_by_cluster,
            decisions,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "InlineBoxApplicationResult(",
            "clusters=",
            {
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
    },
            ", ",
            "advanceByCluster=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.advance_by_cluster).clone();
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
            "decisions=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decisions).clone();
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

#[derive(Clone, Copy)]
pub struct PunctuationGeometryStage;

impl PunctuationGeometryStage {
    pub fn punctuation_geometry_stage_inline_object_attached_marks(c: &Vec<Cluster>, cluster_roles: &Vec<FontRole>, level: KinsokuLevel, kinsoku_rule: Box<dyn KinsokuRule>) -> Vec<InlineObjectAttachedMark> {
        if level == KinsokuLevel::None {
            return vec![];
        }
        let mut result: Vec<InlineObjectAttachedMark> = vec![];
        let mut mark_index = 1u32;
        while (i32::from_ne_bytes((mark_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let mark = (c[usize::try_from(mark_index).unwrap_or(0)]).clone();
            let role = if i32::from_ne_bytes((mark_index).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((cluster_roles.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(cluster_roles[usize::try_from(mark_index).unwrap_or(0)]) } else { None };
            let is_cjk_forbidden = PunctuationGeometryStage::punctuation_geometry_stage_is_cjk_kinsoku_role(role) && kinsoku_rule.forbidden_at_line_start((mark).clone());
            let is_attached_ascii_point_mark = role == Some(FontRole::LatinText) && PunctuationGeometryStage::punctuation_geometry_stage_is_ascii_point_mark_char(PunctuationGeometryStage::punctuation_geometry_stage_first_char((mark.text).to_string().as_str()).clone());
            if !is_cjk_forbidden && !is_attached_ascii_point_mark {
                mark_index = u32::wrapping_add(mark_index, 1);
                continue;
            }
            let mut previous_index = i32::wrapping_sub(i32::from_ne_bytes((mark_index).to_ne_bytes()), 1);
            let mut separator_indices: Vec<u32> = vec![];
            while (previous_index) >= 0 && PunctuationGeometryStage::punctuation_geometry_stage_is_space_run((c[usize::try_from(previous_index).unwrap_or(0)]).clone()) && ((c[usize::try_from(previous_index).unwrap_or(0)]).clone().range).clone().end ==
((c[usize::try_from(i32::wrapping_add(previous_index, 1)).unwrap_or(0)]).clone().range).clone().start {
                separator_indices.insert(0, u32::from_ne_bytes((previous_index).to_ne_bytes()));
                previous_index = i32::wrapping_sub(previous_index, 1);
            }
            if previous_index < (0) {
                mark_index = u32::wrapping_add(mark_index, 1);
                continue;
            }
            let previous = (c[usize::try_from(previous_index).unwrap_or(0)]).clone();
            if !PunctuationGeometryStage::punctuation_geometry_stage_is_inline_object_cluster((previous).clone()) || (previous.range).clone().end != ((c[usize::try_from(i32::wrapping_add(previous_index, 1)).unwrap_or(0)]).clone().range).clone().start {
                mark_index = u32::wrapping_add(mark_index, 1);
                continue;
            }
            result.push(InlineObjectAttachedMark::new(u32::from_ne_bytes((previous_index).to_ne_bytes()), separator_indices.to_vec(), mark_index));
            mark_index = u32::wrapping_add(mark_index, 1);
        }
        return result;
    }

    pub fn punctuation_geometry_stage_inline_object_attached_kinsoku(c: &Vec<Cluster>, attachments: &Vec<InlineObjectAttachedMark>, line_break_clusters: &Vec<Cluster>, level: KinsokuLevel, body_line_width: f64, first_line_width: f64) -> Result<ContextualKinsoku, TextRangeError> {
        if level == KinsokuLevel::None {
            return Ok(PunctuationGeometryStage::punctuation_geometry_stage_empty_contextual_kinsoku());
        }
        if u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((line_break_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "Inline-object kinsoku requires cluster-for-cluster line-break geometry".to_string() });
        }
        let mut forbidden: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut unbreakable_ranges: Vec<IntRange> = vec![];
        let mut forced_hangable: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut extendable_hang_ranges: Vec<IntRange> = vec![];
        let mut decisions: Vec<ContextualKinsokuDecisionInfo> = vec![];
        for attachment in attachments {
            let previous_index = attachment.object_cluster_index;
            let index = attachment.mark_cluster_index;
            let mark = (c[usize::try_from(index).unwrap_or(0)]).clone();
            let is_attached_ascii_point_mark = PunctuationGeometryStage::punctuation_geometry_stage_is_ascii_point_mark_char(PunctuationGeometryStage::punctuation_geometry_stage_first_char((mark.text).to_string().as_str()).clone());
            let mut si = 0u32;
            while (i32::from_ne_bytes((si).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from(((attachment.separator_cluster_indices).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                forbidden.put(&(attachment.separator_cluster_indices[usize::try_from(si).unwrap_or(0)]));
                si = u32::wrapping_add(si, 1);
            }
            forbidden.put(&(index));
            let protected_pair = IntRange::new(previous_index, index);
            let mut pair_terms: Vec<f64> = vec![];
            let mut wi = previous_index;
            while (i32::from_ne_bytes((wi).to_ne_bytes())) <= i32::from_ne_bytes((index).to_ne_bytes()) {
                pair_terms.push(line_break_clusters[usize::try_from(wi).unwrap_or(0)].advance);
                wi = u32::wrapping_add(wi, 1);
            }
            let pair_width = AccurateSum::accurate_sum_of(&pair_terms);
            let available_width = if previous_index == 0 { first_line_width } else { body_line_width };
            if pair_width <= available_width {
                unbreakable_ranges.push(protected_pair.clone());
            } else {
                let may_hang = PunctuationGeometryStage::punctuation_geometry_stage_is_hangable_punctuation_mark((mark.display_text).to_string().as_str()) || is_attached_ascii_point_mark;
                if may_hang {
                    let mut sj = 0u32;
                    while (i32::from_ne_bytes((sj).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from(((attachment.separator_cluster_indices).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        forced_hangable.put(&(attachment.separator_cluster_indices[usize::try_from(sj).unwrap_or(0)]));
                        sj = u32::wrapping_add(sj, 1);
                    }
                    forced_hangable.put(&(index));
                    extendable_hang_ranges.push(protected_pair.clone());
                }
            }
            decisions.push(ContextualKinsokuDecisionInfo::new((mark.range).clone(), (mark.text).to_string().as_str(), index, "LineStart", if u32::try_from(((attachment.separator_cluster_indices).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
"InlineObjectAttachedKinsoku".to_string() } else { "InlineObjectAttachedKinsokuAcrossCollapsedSeparatorSpace".to_string() }.as_str(), None));
        }
        return Ok(ContextualKinsoku::new(forbidden.clone().build(), unbreakable_ranges.to_vec(), forced_hangable.clone().build(), extendable_hang_ranges.to_vec(), decisions.to_vec()));
    }

    pub fn punctuation_geometry_stage_attached_ascii_point_mark_kinsoku(c: &Vec<Cluster>, cluster_roles: &Vec<FontRole>, line_break_clusters: &Vec<Cluster>, level: KinsokuLevel, body_line_width: f64, first_line_width: f64) -> Result<ContextualKinsoku, TextRangeError> {
        if level == KinsokuLevel::None {
            return Ok(PunctuationGeometryStage::punctuation_geometry_stage_empty_contextual_kinsoku());
        }
        if u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((line_break_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "Contextual kinsoku requires cluster-for-cluster line-break geometry".to_string() });
        }
        let mut forbidden: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut unbreakable_ranges: Vec<IntRange> = vec![];
        let mut forced_hangable: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut extendable_hang_ranges: Vec<IntRange> = vec![];
        let mut decisions: Vec<ContextualKinsokuDecisionInfo> = vec![];
        let mut index = 1u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let cluster = (c[usize::try_from(index).unwrap_or(0)]).clone();
            let previous = (c[usize::try_from(u32::wrapping_sub(index, 1)).unwrap_or(0)]).clone();
            let starts_attached_point_mark_run = (if i32::from_ne_bytes((index).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((cluster_roles.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(cluster_roles[usize::try_from(index).unwrap_or(0)]) } else { None }) ==
Some(FontRole::LatinText) && PunctuationGeometryStage::punctuation_geometry_stage_is_ascii_point_mark_char(PunctuationGeometryStage::punctuation_geometry_stage_first_char((cluster.text).to_string().as_str()).clone()) &&
(i32::from_ne_bytes((u_string::unit_count(&((previous.display_text).to_string()))).to_ne_bytes())) > (0) && (i32::from_ne_bytes((u_string::unit_count(&((previous.text).to_string()))).to_ne_bytes())) > (0) &&
!PunctuationGeometryStage::punctuation_geometry_stage_is_whitespace_code(*(u_string::unit_at(&(previous.text).to_string(), u32::wrapping_sub(u_string::unit_count(&((previous.text).to_string())), 1))).as_ref().unwrap()) && (previous.range).clone().end ==
(cluster.range).clone().start;
            if !starts_attached_point_mark_run {
                index = u32::wrapping_add(index, 1);
                continue;
            }
            let run_start = index;
            let mut run_end = index;
            while (i32::from_ne_bytes((u32::wrapping_add(run_end, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                let next = (c[usize::try_from(u32::wrapping_add(run_end, 1)).unwrap_or(0)]).clone();
                let continues_run = (if i32::from_ne_bytes((u32::wrapping_add(run_end, 1)).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((cluster_roles.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(cluster_roles[usize::try_from(u32::wrapping_add(run_end,
1)).unwrap_or(0)]) } else { None }) == Some(FontRole::LatinText) && PunctuationGeometryStage::punctuation_geometry_stage_is_ascii_point_mark_char(PunctuationGeometryStage::punctuation_geometry_stage_first_char((next.text).to_string().as_str()).clone()) &&
((c[usize::try_from(run_end).unwrap_or(0)]).clone().range).clone().end == (next.range).clone().start;
                if !continues_run {
                    break;
                }
                run_end = u32::wrapping_add(run_end, 1);
            }
            let mut fi = run_start;
            while (i32::from_ne_bytes((fi).to_ne_bytes())) <= i32::from_ne_bytes((run_end).to_ne_bytes()) {
                forbidden.put(&(fi));
                fi = u32::wrapping_add(fi, 1);
            }
            unbreakable_ranges.push(IntRange::new(u32::wrapping_sub(run_start, 1), run_end));
            let run_line_width = if u32::wrapping_sub(run_start, 1) == 0 { first_line_width } else { body_line_width };
            let mut run_terms: Vec<f64> = vec![];
            let mut wi = u32::wrapping_sub(run_start, 1);
            while (i32::from_ne_bytes((wi).to_ne_bytes())) <= i32::from_ne_bytes((run_end).to_ne_bytes()) {
                run_terms.push(line_break_clusters[usize::try_from(wi).unwrap_or(0)].advance);
                wi = u32::wrapping_add(wi, 1);
            }
            let run_width = AccurateSum::accurate_sum_of(&run_terms);
            if run_width > (run_line_width) {
                fi = run_start;
                while (i32::from_ne_bytes((fi).to_ne_bytes())) <= i32::from_ne_bytes((run_end).to_ne_bytes()) {
                    forced_hangable.put(&(fi));
                    fi = u32::wrapping_add(fi, 1);
                }
                extendable_hang_ranges.push(IntRange::new(u32::wrapping_sub(run_start, 1), run_end));
            }
            fi = run_start;
            while (i32::from_ne_bytes((fi).to_ne_bytes())) <= i32::from_ne_bytes((run_end).to_ne_bytes()) {
                let point_mark = (c[usize::try_from(fi).unwrap_or(0)]).clone();
                decisions.push(ContextualKinsokuDecisionInfo::new((point_mark.range).clone(), (point_mark.text).to_string().as_str(), fi, "LineStart", "AttachedAsciiPointMarkKinsoku", None));
                fi = u32::wrapping_add(fi, 1);
            }
            index = u32::wrapping_add(run_end, 1);
        }
        return Ok(ContextualKinsoku::new(forbidden.clone().build(), unbreakable_ranges.to_vec(), forced_hangable.clone().build(), extendable_hang_ranges.to_vec(), decisions.to_vec()));
    }

    pub fn punctuation_geometry_stage_is_cjk_kinsoku_role(role: Option<FontRole>) -> bool {
        return role == Some(FontRole::CjkPunctuation);
    }

    pub fn punctuation_geometry_stage_punctuation_atoms(c: Cluster, em: f64, builder: PunctuationAtomBuilder, shaped_glyphs: &Vec<Glyph>, glue_placement: PunctuationGluePlacement, width_policy: PunctuationWidthPolicy) -> Result<Vec<PunctuationAtom>, TextRangeError> {
        let mut out: Vec<PunctuationAtom> = vec![];
        if u_string::unit_count(&((c.display_text).to_string())) == 0 {
            return Ok(out);
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&((c.display_text).to_string()))).to_ne_bytes())) {
            let atom = builder.build(u_string::substring(&(c.display_text).to_string(), i32::from_ne_bytes((i).to_ne_bytes()), i32::wrapping_add(i32::from_ne_bytes((i).to_ne_bytes()), 1)).as_str(),
PunctuationGeometryStage::punctuation_geometry_stage_display_char_source_range((c).clone(), i)?, em, PunctuationGeometryStage::punctuation_geometry_stage_punctuation_ink_input_for((c).clone(), i, &shaped_glyphs)?, Some(glue_placement), Some((width_policy).clone()))?;
            match &(atom) {
                Some(__option) => {
                    out.push((__option).clone());
                }
                None => {
                }
            }
            i = u32::wrapping_add(i, 1);
        }
        return Ok(out);
    }

    pub(crate) fn punctuation_geometry_stage_punctuation_ink_input_for(c: Cluster, display_index: u32, shaped_glyphs: &Vec<Glyph>) -> Result<Option<PunctuationInkInput>, TextRangeError> {
        if u32::try_from((shaped_glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(None);
        }
        let mut glyph: Option<Glyph> = None;
        if u32::try_from((shaped_glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) == u_string::unit_count(&((c.display_text).to_string())) {
            let source = (shaped_glyphs[usize::try_from(display_index).unwrap_or(0)]).clone();
            let mut pen_terms: Vec<f64> = vec![];
            let mut i = 0u32;
            while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((display_index).to_ne_bytes())) {
                pen_terms.push(shaped_glyphs[usize::try_from(i).unwrap_or(0)].advance);
                i = u32::wrapping_add(i, 1);
            }
            let character_pen = AccurateSum::accurate_sum_of(&pen_terms);
            glyph = Some(Glyph::new(source.id, (source.cluster_range).clone(), source.advance, Some(source.x - character_pen), Some(source.y), source.render_font_key.clone(), (source.bounds).clone(), source.halt_advance, source.halt_placement_x).clone());
        } else {
            if u_string::unit_count(&((c.display_text).to_string())) == 1 {
                glyph = PunctuationGeometryStage::punctuation_geometry_stage_union_as_single_glyph(&shaped_glyphs);
            }
        }
        if glyph.is_none() {
            return Ok(Some(PunctuationInkInput::new(0 as f64 as f64, None, None, None, Some("glyph-cluster-mapping-ambiguous".to_string()))?));
        }
        return Ok(Some(PunctuationInkInput::new((glyph).as_ref().unwrap().advance, match &((glyph).as_ref().unwrap().bounds) { None => None, Some(__option1) => Some(Rect::new(__option1.left + (glyph).as_ref().unwrap().x, __option1.top + (glyph).as_ref().unwrap().y,
__option1.right + (glyph).as_ref().unwrap().x, __option1.bottom + (glyph).as_ref().unwrap().y)) }, (glyph).as_ref().unwrap().halt_advance, (glyph).as_ref().unwrap().halt_placement_x, if glyph.as_ref().unwrap().bounds.clone().is_none() { Some("shaper-no-ink-bounds".to_string()) }
else { None }.clone())?));
    }

    pub(crate) fn punctuation_geometry_stage_union_as_single_glyph(g: &Vec<Glyph>) -> Option<Glyph> {
        if u32::try_from((g.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return None;
        }
        let first = (g[0usize]).clone();
        let mut bounds: Vec<Rect> = vec![];
        let mut union_terms: Vec<f64> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((g.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let glyph = (g[usize::try_from(i).unwrap_or(0)]).clone();
            union_terms.push(glyph.advance);
            match &(glyph.bounds) {
                Some(__option5) => {
                    bounds.push(Rect::new(__option5.left + glyph.x, __option5.top + glyph.y, __option5.right + glyph.x, __option5.bottom + glyph.y));
                }
                None => {
                }
            }
            i = u32::wrapping_add(i, 1);
        }
        if u32::try_from((bounds.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Some(first);
        }
        let total_advance = AccurateSum::accurate_sum_of(&union_terms);
        let mut left = bounds[0usize].left;
        let mut top = bounds[0usize].top;
        let mut right = bounds[0usize].right;
        let mut bottom = bounds[0usize].bottom;
        i = 1u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((bounds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            left = { let __min_a = left as f64; let __min_b = bounds[usize::try_from(i).unwrap_or(0)].left as f64; if __min_a.is_nan() || __min_b.is_nan() { f64::NAN } else { if __min_a < __min_b { __min_a } else if __min_b < __min_a { __min_b } else if __min_a == 0.0 && __min_b
== 0.0 { if __min_a.is_sign_negative() { __min_a } else { __min_b } } else { __min_a } } };
            top = { let __min_a1 = top as f64; let __min_b1 = bounds[usize::try_from(i).unwrap_or(0)].top as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 < __min_b1 { __min_a1 } else if __min_b1 < __min_a1 { __min_b1 } else if __min_a1 == 0.0 &&
__min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_a1 } else { __min_b1 } } else { __min_a1 } } };
            right = { let __min_a2 = right as f64; let __min_b2 = bounds[usize::try_from(i).unwrap_or(0)].right as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 ==
0.0 && __min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } };
            bottom = { let __min_a3 = bottom as f64; let __min_b3 = bounds[usize::try_from(i).unwrap_or(0)].bottom as f64; if __min_a3.is_nan() || __min_b3.is_nan() { f64::NAN } else { if __min_a3 > __min_b3 { __min_a3 } else if __min_b3 > __min_a3 { __min_b3 } else if __min_a3
== 0.0 && __min_b3 == 0.0 { if __min_a3.is_sign_negative() { __min_b3 } else { __min_a3 } } else { __min_a3 } } };
            i = u32::wrapping_add(i, 1);
        }
        return Some(Glyph::new(first.id, (first.cluster_range).clone(), total_advance, Some(0 as f64), Some(0 as f64), first.render_font_key.clone(), Some(Rect::new(left, top, right, bottom)), None, None));
    }

    pub(crate) fn punctuation_geometry_stage_display_char_source_range(c: Cluster, display_index: u32) -> Result<TextRange, TextRangeError> {
        return Ok(if u_string::unit_count(&((c.display_text).to_string())) == u_string::unit_count(&((c.text).to_string())) { TextRange::new(u32::wrapping_add((c.range).clone().start, display_index), u32::wrapping_add(u32::wrapping_add((c.range).clone().start, display_index),
1))? } else { (c.range).clone() });
    }

    pub fn punctuation_geometry_stage_is_space_run(c: Cluster) -> bool {
        if u_string::unit_count(&((c.text).to_string())) == 0 {
            return false;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&((c.text).to_string()))).to_ne_bytes())) {
            if u_string::substring(&(c.text).to_string(), i32::from_ne_bytes((i).to_ne_bytes()), i32::wrapping_add(i32::from_ne_bytes((i).to_ne_bytes()), 1)) != " " {
                return false;
            }
            i = u32::wrapping_add(i, 1);
        }
        return true;
    }

    pub fn punctuation_geometry_stage_apply_auto_space_policy(c: &Vec<Cluster>, east_asian_spacing_edges: &Vec<EastAsianSpacingEdges>, inline_attachments: &Vec<InlineAttachment>, policy: AutoSpacePolicy, font_size: f64, narrow_inline_box_leading_clusters:
Option<SortedSetTable<u32>>, narrow_inline_box_trailing_clusters: Option<SortedSetTable<u32>>) -> Result<AutoSpaceApplicationResult, TextRangeError> {
        if u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(AutoSpaceApplicationResult::new(vec![].to_vec(), vec![].to_vec()));
        }
        if u32::try_from((east_asian_spacing_edges.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "East_Asian_Spacing values must align with natural clusters.".to_string() });
        }
        if u32::try_from((inline_attachments.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "Inline attachments must align with natural clusters.".to_string() });
        }
        let narrow_leading = match &(narrow_inline_box_leading_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
Some(__option6) => (*__option6).clone() };
        let narrow_trailing = match &(narrow_inline_box_trailing_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option7) => (*__option7).clone() };
        let mut decisions: Vec<AutoSpaceDecisionInfo> = vec![];
        let gap = policy.gap_em * font_size;
        let attached_boundaries = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&inline_attachments);
        let mut suppressed: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut bi = 0u32;
        while (i32::from_ne_bytes((bi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((attached_boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let boundary = (attached_boundaries[usize::try_from(bi).unwrap_or(0)]).clone();
            bi = u32::wrapping_add(bi, 1);
            suppressed.put(&(boundary.previous_cluster_index));
            if boundary.next_cluster_index.is_some() {
                suppressed.put(&(boundary.attached_cluster_range.end));
            }
        }
        let mut virtual_gap_at_run_end: Vec<bool> = vec![];
        for _ in 0..match u32::try_from(c.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            virtual_gap_at_run_end.push(false);
        }
        bi = 0u32;
        while (i32::from_ne_bytes((bi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((attached_boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let boundary = (attached_boundaries[usize::try_from(bi).unwrap_or(0)]).clone();
            bi = u32::wrapping_add(bi, 1);
            if boundary.next_cluster_index.is_none() {
                continue;
            }
            let next_index = boundary.next_cluster_index;
            if i32::from_ne_bytes((*(next_index).as_ref().unwrap()).to_ne_bytes()) >= i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) {
                continue;
            }
            let next_cluster = (c[usize::try_from(*((next_index).as_ref().unwrap())).unwrap_or(0)]).clone();
            if PunctuationGeometryStage::punctuation_geometry_stage_is_space_run((next_cluster).clone()) || PunctuationGeometryStage::punctuation_geometry_stage_is_mandatory_break_cluster((next_cluster).clone()) {
                continue;
            }
            let previous_index = boundary.previous_cluster_index;
            let previous_edge = east_asian_spacing_edges[usize::try_from(previous_index).unwrap_or(0)].trailing;
            let next_edge = east_asian_spacing_edges[usize::try_from(*((next_index).as_ref().unwrap())).unwrap_or(0)].leading;
            let mut narrow_char: Option<String> = None;
            if previous_edge == EastAsianSpacingValue::Wide && next_edge == EastAsianSpacingValue::Narrow {
                narrow_char = PunctuationGeometryStage::punctuation_geometry_stage_first_char((next_cluster.text).to_string().as_str());
            } else {
                if previous_edge == EastAsianSpacingValue::Narrow && next_edge == EastAsianSpacingValue::Wide {
                    narrow_char = PunctuationGeometryStage::punctuation_geometry_stage_last_char(((c[usize::try_from(previous_index).unwrap_or(0)]).clone().text).to_string().as_str());
                }
            }
            { while virtual_gap_at_run_end.len() <= usize::try_from(boundary.attached_cluster_range.end).unwrap_or(0) { virtual_gap_at_run_end.push(false); } virtual_gap_at_run_end[usize::try_from(boundary.attached_cluster_range.end).unwrap_or(0)] =
PunctuationGeometryStage::punctuation_geometry_stage_mode_for_narrow((policy).clone(), narrow_char.clone()) == Some(AutoSpaceMode::Insert); };
        }
        let suppressed_set: SortedSetTable<u32> = suppressed.clone().build();
        let mut updated: Vec<Cluster> = vec![];
        let mut idx = 0u32;
        while (i32::from_ne_bytes((idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let cluster = (c[usize::try_from(idx).unwrap_or(0)]).clone();
            let previous_spacing = if u32::wrapping_sub(idx, 1) <= 2147483647 { Some(east_asian_spacing_edges[usize::try_from(u32::wrapping_sub(idx, 1)).unwrap_or(0)].trailing) } else { None };
            let current_spacing = (east_asian_spacing_edges[usize::try_from(idx).unwrap_or(0)]).clone();
            let next_spacing = if i32::from_ne_bytes((u32::wrapping_add(idx, 1)).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((east_asian_spacing_edges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(east_asian_spacing_edges[usize::try_from(u32::wrapping_add(idx,
1)).unwrap_or(0)].leading) } else { None };
            if PunctuationGeometryStage::punctuation_geometry_stage_is_space_run((cluster).clone()) {
                let mut narrow_boundary_char: Option<String> = None;
                if previous_spacing == Some(EastAsianSpacingValue::Wide) && next_spacing == Some(EastAsianSpacingValue::Narrow) {
                    narrow_boundary_char = if i32::from_ne_bytes((u32::wrapping_add(idx, 1)).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
PunctuationGeometryStage::punctuation_geometry_stage_first_char(((c[usize::try_from(u32::wrapping_add(idx, 1)).unwrap_or(0)]).clone().text).to_string().as_str()) } else { None };
                } else {
                    if previous_spacing == Some(EastAsianSpacingValue::Narrow) && next_spacing == Some(EastAsianSpacingValue::Wide) {
                        narrow_boundary_char = if u32::wrapping_sub(idx, 1) <= 2147483647 { PunctuationGeometryStage::punctuation_geometry_stage_last_char(((c[usize::try_from(u32::wrapping_sub(idx, 1)).unwrap_or(0)]).clone().text).to_string().as_str()) } else { None };
                    }
                }
                let mode = PunctuationGeometryStage::punctuation_geometry_stage_mode_for_narrow((policy).clone(), narrow_boundary_char.clone());
                if match &(mode) { None => true, Some(__option9) => Some(*__option9) == Some(AutoSpaceMode::Disabled) } {
                    updated.push(cluster.clone());
                    idx = u32::wrapping_add(idx, 1);
                    continue;
                }
                let reduction = cluster.advance - gap;
                if reduction == 0 as f64 {
                    updated.push(cluster.clone());
                    idx = u32::wrapping_add(idx, 1);
                    continue;
                }
                decisions.push(AutoSpaceDecisionInfo::new((cluster.range).clone(), "gap", "EastAsianSpacing.Wide", AutoSpaceMode::Replace.name().to_string().as_str(), u_string::unit_count(&((cluster.text).to_string())), reduction / format!("{}",
(i32::from_ne_bytes((u_string::unit_count(&((cluster.text).to_string()))).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0), reduction, "TextAutoSpaceReplace:east-asian-spacing-W-space-N"));
                updated.push(Cluster::new((cluster.range).clone(), (cluster.text).to_string().as_str(), (cluster.font_key).to_string().as_str(), gap, Some((cluster.display_text).to_string()), Some(cluster.baseline_shift), Some(cluster.leading_layout_advance),
Some(cluster.glyph_inline_shift)));
                idx = u32::wrapping_add(idx, 1);
                continue;
            }
            let mut added = 0.0f64;
            if previous_spacing == Some(EastAsianSpacingValue::Wide) && current_spacing.leading == EastAsianSpacingValue::Narrow && PunctuationGeometryStage::punctuation_geometry_stage_mode_for_narrow((policy).clone(),
PunctuationGeometryStage::punctuation_geometry_stage_first_char((cluster.text).to_string().as_str()).clone()) == Some(AutoSpaceMode::Insert) && !suppressed_set.has(&(u32::wrapping_sub(idx, 1))) {
                added += gap;
                decisions.push(AutoSpaceDecisionInfo::new((cluster.range).clone(), "leading", if narrow_leading.has(&(idx)) { "InlineBox.Narrow".to_string() } else { "EastAsianSpacing.Wide".to_string() }.as_str(), AutoSpaceMode::Insert.name().to_string().as_str(), 0u32, 0 as f64
as f64, -gap, if narrow_leading.has(&(idx)) { "InlineBoxOuterAutoSpace:leading-W-N".to_string() } else { "TextAutoSpaceInsert:east-asian-spacing-W-N".to_string() }.as_str()));
            }
            let normal_trailing_gap = next_spacing == Some(EastAsianSpacingValue::Wide) && current_spacing.trailing == EastAsianSpacingValue::Narrow && PunctuationGeometryStage::punctuation_geometry_stage_mode_for_narrow((policy).clone(),
PunctuationGeometryStage::punctuation_geometry_stage_last_char((cluster.text).to_string().as_str()).clone()) == Some(AutoSpaceMode::Insert) && !suppressed_set.has(&(idx));
            let virtual_trailing_gap = virtual_gap_at_run_end[usize::try_from(idx).unwrap_or(0)];
            if normal_trailing_gap || virtual_trailing_gap {
                added += gap;
                decisions.push(AutoSpaceDecisionInfo::new((cluster.range).clone(), "trailing", if narrow_trailing.has(&(idx)) { "InlineBox.Narrow".to_string() } else { if virtual_trailing_gap { "InlineAttachment.Previous".to_string() } else { "EastAsianSpacing.Wide".to_string()
}.to_string() }.as_str(), AutoSpaceMode::Insert.name().to_string().as_str(), 0u32, 0 as f64 as f64, -gap, if narrow_trailing.has(&(idx)) { "InlineBoxOuterAutoSpace:trailing-N-W".to_string() } else { if virtual_trailing_gap {
"AttachedInlineVirtualAutoSpace:east-asian-spacing-W-N".to_string() } else { "TextAutoSpaceInsert:east-asian-spacing-W-N".to_string() }.to_string() }.as_str()));
            }
            if added == 0 as f64 {
                updated.push(cluster.clone());
            } else {
                updated.push(Cluster::new((cluster.range).clone(), (cluster.text).to_string().as_str(), (cluster.font_key).to_string().as_str(), cluster.advance + added, Some((cluster.display_text).to_string()), Some(cluster.baseline_shift), Some(cluster.leading_layout_advance),
Some(cluster.glyph_inline_shift)));
            }
            idx = u32::wrapping_add(idx, 1);
        }
        return Ok(AutoSpaceApplicationResult::new(updated.to_vec(), decisions.to_vec()));
    }

    pub fn punctuation_geometry_stage_is_east_asian_spacing_boundary_at(right_index: u32, clusters: &Vec<Cluster>, spacing_edges: &Vec<EastAsianSpacingEdges>) -> bool {
        let left_index = u32::wrapping_sub(right_index, 1);
        let left = spacing_edges[usize::try_from(left_index).unwrap_or(0)].trailing;
        let right = spacing_edges[usize::try_from(right_index).unwrap_or(0)].leading;
        if PunctuationGeometryStage::punctuation_geometry_stage_is_wide_narrow_pair_with(left, right) {
            return true;
        }
        if PunctuationGeometryStage::punctuation_geometry_stage_is_space_run((clusters[usize::try_from(right_index).unwrap_or(0)]).clone()) && left == EastAsianSpacingValue::Wide && (if i32::from_ne_bytes((u32::wrapping_add(right_index, 1)).to_ne_bytes()) <
(i32::from_ne_bytes((u32::try_from((spacing_edges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(spacing_edges[usize::try_from(u32::wrapping_add(right_index, 1)).unwrap_or(0)].leading) } else { None }) == Some(EastAsianSpacingValue::Narrow) {
            return true;
        }
        if PunctuationGeometryStage::punctuation_geometry_stage_is_space_run((clusters[usize::try_from(left_index).unwrap_or(0)]).clone()) && right == EastAsianSpacingValue::Wide && (if u32::wrapping_sub(left_index, 1) <= 2147483647 {
Some(spacing_edges[usize::try_from(u32::wrapping_sub(left_index, 1)).unwrap_or(0)].trailing) } else { None }) == Some(EastAsianSpacingValue::Narrow) {
            return true;
        }
        return false;
    }

    pub(crate) fn punctuation_geometry_stage_is_wide_narrow_pair_with(v: EastAsianSpacingValue, other: EastAsianSpacingValue) -> bool {
        return v == EastAsianSpacingValue::Wide && other == EastAsianSpacingValue::Narrow || v == EastAsianSpacingValue::Narrow && other == EastAsianSpacingValue::Wide;
    }

    pub fn punctuation_geometry_stage_is_attached_ascii_point_mark_at(c: &Vec<Cluster>, index: u32) -> bool {
        if i32::from_ne_bytes((index).to_ne_bytes()) <= 0 {
            return false;
        }
        let cluster = (c[usize::try_from(index).unwrap_or(0)]).clone();
        let previous = (c[usize::try_from(u32::wrapping_sub(index, 1)).unwrap_or(0)]).clone();
        return PunctuationGeometryStage::punctuation_geometry_stage_is_ascii_point_mark_char(PunctuationGeometryStage::punctuation_geometry_stage_first_char((cluster.text).to_string().as_str()).clone()) &&
(i32::from_ne_bytes((u_string::unit_count(&((previous.display_text).to_string()))).to_ne_bytes())) > (0) && (i32::from_ne_bytes((u_string::unit_count(&((previous.text).to_string()))).to_ne_bytes())) > (0) &&
!PunctuationGeometryStage::punctuation_geometry_stage_is_whitespace_code(*(u_string::unit_at(&(previous.text).to_string(), u32::wrapping_sub(u_string::unit_count(&((previous.text).to_string())), 1))).as_ref().unwrap()) && (previous.range).clone().end ==
(cluster.range).clone().start;
    }

    pub fn punctuation_geometry_stage_apply_inline_box_spans(c: &Vec<Cluster>, spans: &Vec<InlineBoxSpan>) -> InlineBoxApplicationResult {
        if u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || u32::try_from((spans.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            let empty_advance: SortedMapTable<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build();
            return InlineBoxApplicationResult::new(c.to_vec(), (empty_advance).clone(), vec![].to_vec());
        }
        let mut leading_by_cluster: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut trailing_by_cluster: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut decisions: Vec<InlineBoxDecisionInfo> = vec![];
        for span in spans {
            if i32::from_ne_bytes(((span.range).clone().start).to_ne_bytes()) >= i32::from_ne_bytes(((span.range).clone().end).to_ne_bytes()) {
                continue;
            }
            let cluster_range = PunctuationGeometryStage::punctuation_geometry_stage_cluster_index_range_for(&c, (span.range).clone());
            if cluster_range.is_none() {
                continue;
            }
            if span.inline_start != 0 as f64 {
                let existing = leading_by_cluster.get(&((cluster_range).as_ref().unwrap().start));
                leading_by_cluster.put(&((cluster_range).as_ref().unwrap().start), &(match &(existing) { None => span.inline_start, Some(__option11) => *__option11 + span.inline_start }));
            }
            if span.inline_end != 0 as f64 {
                let existing = trailing_by_cluster.get(&((cluster_range).as_ref().unwrap().end));
                trailing_by_cluster.put(&((cluster_range).as_ref().unwrap().end), &(match &(existing) { None => span.inline_end, Some(__option13) => *__option13 + span.inline_end }));
            }
            decisions.push(InlineBoxDecisionInfo::new((span.range).clone(), span.inline_start, span.inline_end, span.outer_spacing.name().to_string().as_str(), (cluster_range).as_ref().unwrap().start, (cluster_range).as_ref().unwrap().end,
Some("InlineBoxBoundaryAdvance".to_string())));
        }
        let mut advance_by_cluster: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut resolved: Vec<Cluster> = vec![];
        let mut idx = 0u32;
        while (i32::from_ne_bytes((idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let cluster = (c[usize::try_from(idx).unwrap_or(0)]).clone();
            let leading = leading_by_cluster.get(&(idx));
            let trailing = trailing_by_cluster.get(&(idx));
            let lead = match &(leading) { None => 0.0f64, Some(__option14) => *__option14 };
            let trail = match &(trailing) { None => 0.0f64, Some(__option15) => *__option15 };
            let structural = lead + trail;
            if structural != 0 as f64 {
                advance_by_cluster.put(&(idx), &(structural));
            }
            if structural == 0 as f64 && lead == 0 as f64 {
                resolved.push(cluster.clone());
            } else {
                resolved.push(Cluster::new((cluster.range).clone(), (cluster.text).to_string().as_str(), (cluster.font_key).to_string().as_str(), (if cluster.advance + structural < (0 as f64) { 0 as f64 } else { cluster.advance + structural } as f64) as f64,
Some((cluster.display_text).to_string()), Some(cluster.baseline_shift), Some(cluster.leading_layout_advance + lead), Some(cluster.glyph_inline_shift)));
            }
            idx = u32::wrapping_add(idx, 1);
        }
        return InlineBoxApplicationResult::new(resolved.to_vec(), advance_by_cluster.clone().build(), decisions.to_vec());
    }

    pub(crate) fn punctuation_geometry_stage_cluster_index_range_for(c: &Vec<Cluster>, source_range: TextRange) -> Option<IntRange> {
        if u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return None;
        }
        let mut low = 0u32;
        let mut high = u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i32::from_ne_bytes((low).to_ne_bytes())) < (i32::from_ne_bytes((high).to_ne_bytes())) {
            let mid = u32::wrapping_add(low, high) >> 1;
            if i32::from_ne_bytes((((c[usize::try_from(mid).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()) < (i32::from_ne_bytes((source_range.start).to_ne_bytes())) {
                low = u32::wrapping_add(mid, 1);
            } else {
                high = mid;
            }
        }
        let first = low;
        low = first;
        high = u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i32::from_ne_bytes((low).to_ne_bytes())) < (i32::from_ne_bytes((high).to_ne_bytes())) {
            let mid = u32::wrapping_add(low, high) >> 1;
            if i32::from_ne_bytes((((c[usize::try_from(mid).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes()) <= i32::from_ne_bytes((source_range.end).to_ne_bytes()) {
                low = u32::wrapping_add(mid, 1);
            } else {
                high = mid;
            }
        }
        let last_exclusive = low;
        return if i32::from_ne_bytes((first).to_ne_bytes()) < (i32::from_ne_bytes((last_exclusive).to_ne_bytes())) { Some(IntRange::new(first, u32::wrapping_sub(last_exclusive, 1))) } else { None };
    }

    pub(crate) fn punctuation_geometry_stage_empty_contextual_kinsoku() -> ContextualKinsoku {
        return ContextualKinsoku::new(SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), vec![].to_vec(),
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), vec![].to_vec(), vec![].to_vec());
    }

    pub(crate) fn punctuation_geometry_stage_mode_for_narrow(policy: AutoSpacePolicy, boundary_char: Option<String>) -> Option<AutoSpaceMode> {
        if boundary_char.is_none() {
            return None;
        }
        return if PunctuationGeometryStage::punctuation_geometry_stage_is_digit_char(u_string::unit_at(&(boundary_char).as_ref().unwrap(), 0u32)) { Some(policy.cjk_digit) } else { Some(policy.cjk_latin) };
    }

    pub(crate) fn punctuation_geometry_stage_is_digit_char(code: Option<u32>) -> bool {
        return (i32::from_ne_bytes((code.unwrap_or(0)).to_ne_bytes())) >= 48 && (i32::from_ne_bytes((code.unwrap_or(0)).to_ne_bytes())) <= 57 || (i32::from_ne_bytes((code.unwrap_or(0)).to_ne_bytes())) >= 65296 && (i32::from_ne_bytes((code.unwrap_or(0)).to_ne_bytes())) <= 65305;
    }

    pub(crate) fn punctuation_geometry_stage_is_ascii_point_mark_char(ch: Option<String>) -> bool {
        return match &(ch) { Some(__option16) => ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark(__option16), None => false };
    }

    pub(crate) fn punctuation_geometry_stage_is_hangable_punctuation_mark(display_text: &str) -> bool {
        if u_string::unit_count(&(display_text)) != 1 {
            return false;
        }
        return display_text == "、" || display_text == "，" || display_text == "。";
    }

    pub(crate) fn punctuation_geometry_stage_is_inline_object_cluster(c: Cluster) -> bool {
        return (c.font_key).to_string() == "inline-object";
    }

    pub(crate) fn punctuation_geometry_stage_is_mandatory_break_cluster(c: Cluster) -> bool {
        return (c.font_key).to_string() == "mandatory-break" && u_string::unit_count(&((c.display_text).to_string())) == 0;
    }

    pub(crate) fn punctuation_geometry_stage_is_whitespace_code(code: u32) -> bool {
        return (i32::from_ne_bytes((code).to_ne_bytes())) >= 9 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 13 || code == 32 || code == 12288;
    }

    pub(crate) fn punctuation_geometry_stage_first_char(s: &str) -> Option<String> {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        return if i32::from_ne_bytes((__count).to_ne_bytes()) > (0) { Some(u_string::char_at_from(&__units, 0u32).to_string()) } else { None };
    }

    pub(crate) fn punctuation_geometry_stage_last_char(s: &str) -> Option<String> {
    let __units1 = u_string::units(&s);
    let __count1 = u_string::unit_count(&s);
        return if i32::from_ne_bytes((__count1).to_ne_bytes()) > (0) { Some(u_string::char_at_from(&__units1, u32::wrapping_sub(__count1, 1)).to_string()) } else { None };
    }
}
