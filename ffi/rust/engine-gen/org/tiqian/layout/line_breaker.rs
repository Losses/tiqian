use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::kinsoku_rule::KinsokuRule;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::line_optimization::RepairCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::line_repair::LineRepair;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;
use std::sync::Arc;


pub trait LineBreaker: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn LineBreaker>;
    fn get_strategy_name(&self) -> UString;
    fn break_lines(&self, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: Option<Vec<ShrinkOpportunity>>, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters: Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: Option<SortedSetTable<u32>>, cjk_inter_char_boundaries: Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>, line_adjustment_push_in: Option<bool>, line_adjustment_compress_bias: Option<f64>, hard_break_after_clusters: Option<SortedSetTable<u32>>, non_rendering_control_clusters: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError>;
}

impl Clone for Box<dyn LineBreaker> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn LineBreaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone)]
pub struct GreedyLineBreaker {
    pub(crate) kinsoku: Box<dyn KinsokuRule>,
    pub(crate) push_in_penalty: u32,
    pub(crate) carry_previous_penalty: u32,
    pub(crate) leave_ragged_penalty: u32,
}

impl GreedyLineBreaker {
    pub fn new(kinsoku: Option<Box<dyn KinsokuRule>>, push_in_penalty: Option<u32>, carry_previous_penalty: Option<u32>, leave_ragged_penalty: Option<u32>) -> Self {
    let mut _field_push_in_penalty = 0;
    let mut _field_leave_ragged_penalty = 0;
    let mut _field_kinsoku: Box<dyn KinsokuRule>;
    let mut _field_carry_previous_penalty = 0;
        match &(kinsoku) {
            None => {
                _field_kinsoku = Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)));
            }
            Some(__option) => {
                _field_kinsoku = __option.clone();
            }
        }
        match &(push_in_penalty) {
            None => {
                _field_push_in_penalty = 2u32;
            }
            Some(__option1) => {
                _field_push_in_penalty = *__option1;
            }
        }
        match &(carry_previous_penalty) {
            None => {
                _field_carry_previous_penalty = 10u32;
            }
            Some(__option2) => {
                _field_carry_previous_penalty = *__option2;
            }
        }
        match &(leave_ragged_penalty) {
            None => {
                _field_leave_ragged_penalty = 20u32;
            }
            Some(__option3) => {
                _field_leave_ragged_penalty = *__option3;
            }
        }
        Self {
            kinsoku: _field_kinsoku,
            push_in_penalty: _field_push_in_penalty,
            carry_previous_penalty: _field_carry_previous_penalty,
            leave_ragged_penalty: _field_leave_ragged_penalty,
        }
    }

    pub fn get_strategy_name(&self) -> UString {
        return UString::from("greedy").to_ustring();
    }

    pub fn break_lines(&self, n: &Vec<Cluster>, a: &Vec<Cluster>, max_width: f64, shrink_opportunities: Option<Vec<ShrinkOpportunity>>, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters: Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: Option<SortedSetTable<u32>>, cjk_inter_char_boundaries: Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>, line_adjustment_push_in: Option<bool>, line_adjustment_compress_bias: Option<f64>, hard_break_after_clusters: Option<SortedSetTable<u32>>, non_rendering_control_clusters: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        if u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
        }
        if u32::try_from((n.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: UString::from("naturalClusters and adjustedClusters must align cluster-for-cluster.") });
        }
        let shrink_ops = match &(shrink_opportunities) { None => vec![], Some(__option4) => (*__option4).clone() };
        let ranges = match &(unbreakable_ranges) { None => (*crate::org::tiqian::layout::progressive_break_decisions::UNBREAKABLE_RANGES_EMPTY).clone(), Some(__option5) => (*__option5).clone() };
        let indent = match &(first_line_indent) { None => 0 as f64, Some(__option6) => *__option6 };
        let hangables = match &(hangable_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option7) => (*__option7).clone() };
        let extendables = match &(extendable_hang_ranges) { None => vec![], Some(__option8) => (*__option8).clone() };
        let forbid_end = match &(forbidden_line_end_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option9) => (*__option9).clone() };
        let hyphens = match &(hyphen_break_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option10) => (*__option10).clone() };
        let cjk = match &(cjk_inter_char_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option11) => (*__option11).clone() };
        let max_stretch = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option12) => *__option12 };
        let sino = match &(sino_western_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option13) => (*__option13).clone() };
        let sino_cap = match &(sino_western_stretch_cap) { None => 0 as f64, Some(__option14) => *__option14 };
        let hard = match &(hard_break_after_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option15) => (*__option15).clone() };
        let controls = match &(non_rendering_control_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option16) => (*__option16).clone() };
        let progressive = match &(progressive_break_opportunities) { None => LineBreakerLines::line_breaker_lines_empty_progressive_map(), Some(__option17) => (*__option17).clone() };
        let greedy = self.greedy_fill(&n, &a, max_width, (ranges).clone(), indent, (forbid_end).clone(), (hyphens).clone(), (cjk).clone(), max_stretch, (sino).clone(), sino_cap, (hard).clone(), (controls).clone(), (progressive).clone())?;
        let repaired = LineRepair::line_repair_apply_kinsoku_repairs(&greedy, &n, &a, max_width, (self.kinsoku).clone(), &shrink_ops, self.push_in_penalty, self.carry_previous_penalty, self.leave_ragged_penalty, Some((ranges).clone()), Some(indent), Some((hangables).clone()), Some((extendables).clone()), Some(5), (forbidden_line_start_clusters).clone())?;
        let mut gap_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut gi = 0u32;
        while (i32::from_ne_bytes(((gi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((cjk.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(cjk.at(i32::from_ne_bytes(((gi) as i32).to_ne_bytes()))));
            gi = u32::wrapping_add(gi, 1);
        }
        let mut si = 0u32;
        while (i32::from_ne_bytes(((si) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((sino.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(sino.at(i32::from_ne_bytes(((si) as i32).to_ne_bytes()))));
            si = u32::wrapping_add(si, 1);
        }
        let gap_boundaries: SortedSetTable<u32> = gap_builder.clone().build();
        let push_in = match &(line_adjustment_push_in) { None => false, Some(__option18) => *__option18 };
        let compress_bias = match &(line_adjustment_compress_bias) { None => 1.0f64, Some(__option19) => *__option19 };
        return Ok(LineRepair::line_repair_with_fill_push_in((repaired).clone(), push_in, &n, &a, max_width, &shrink_ops, indent, compress_bias, (forbidden_line_start_clusters).clone(), (forbid_end).clone(), (ranges).clone(), self.push_in_penalty, Some((gap_boundaries).clone()), Some((progressive).clone()))?);
    }

    fn greedy_fill(&self, n: &Vec<Cluster>, a: &Vec<Cluster>, max_width: f64, unbreakable_ranges: UnbreakableRanges, first_line_indent: f64, forbidden_line_end_clusters: SortedSetTable<u32>, hyphen_break_clusters: SortedSetTable<u32>, cjk_inter_char_boundaries: SortedSetTable<u32>, max_cjk_stretch_per_gap: f64, sino_western_boundaries: SortedSetTable<u32>, sino_western_stretch_cap: f64, hard_break_after_clusters: SortedSetTable<u32>, non_rendering_control_clusters: SortedSetTable<u32>, progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>) -> Result<Vec<LineCandidate>, TextRangeError> {
        let mut lines: Vec<LineCandidate> = vec![];
        let mut line_start = 0u32;
        let mut adjusted_accum = 0.0f64;
        let mut natural_accum = 0.0f64;
        let mut has_rendering_content = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let next_adjusted = adjusted_accum + a[usize::try_from(i).unwrap_or(0)].advance;
            let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, first_line_indent, line_start);
            if next_adjusted > (limit) && has_rendering_content {
                let progressive = ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(line_start, i, (progressive_break_opportunities).clone(), Some((a).clone()), Some(limit), Some((cjk_inter_char_boundaries).clone()), Some(max_cjk_stretch_per_gap), Some((sino_western_boundaries).clone()), Some(sino_western_stretch_cap));
                let decided = ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(line_start, progressive, &a, limit, (hyphen_break_clusters).clone(), (cjk_inter_char_boundaries).clone(), max_cjk_stretch_per_gap, Some((sino_western_boundaries).clone()), Some(sino_western_stretch_cap));
                let after_unbreak = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(decided, line_start, (unbreakable_ranges).clone());
                let break_at = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_line_end(after_unbreak, line_start, (forbidden_line_end_clusters).clone());
                lines.push(LineBreakerLines::line_breaker_lines_close_filled_line(IntRange::new(line_start, u32::wrapping_sub(break_at, 1)), after_unbreak, &n, &a)?);
                line_start = break_at;
                adjusted_accum = a[usize::try_from(break_at).unwrap_or(0)].advance;
                natural_accum = n[usize::try_from(break_at).unwrap_or(0)].advance;
                has_rendering_content = !non_rendering_control_clusters.has(&(break_at));
                i = u32::wrapping_add(break_at, 1);
            } else {
                adjusted_accum = next_adjusted;
                natural_accum += n[usize::try_from(i).unwrap_or(0)].advance;
                if !non_rendering_control_clusters.has(&(i)) {
                    has_rendering_content = true;
                }
                if hard_break_after_clusters.has(&(i)) {
                    lines.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, i), &n, &a, Some(LineEndReason::MandatoryBreak), None, None)?);
                    line_start = u32::wrapping_add(i, 1);
                    adjusted_accum = 0 as f64 as f64;
                    natural_accum = 0 as f64 as f64;
                    has_rendering_content = false;
                }
                i = u32::wrapping_add(i, 1);
            }
        }
        if i32::from_ne_bytes(((line_start) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            lines.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, u32::wrapping_sub(u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)), &n, &a, Some(LineEndReason::ParagraphEnd), None, None)?);
        } else {
            if hard_break_after_clusters.has(&(u32::wrapping_sub(u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), 1))) {
                lines.push(LineBreakerLines::line_breaker_lines_empty_line_candidate(((a[usize::try_from(u32::wrapping_sub(u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end, Some(LineEndReason::ParagraphEnd))?);
            }
        }
        return Ok(lines);
    }
}

impl LineBreaker for GreedyLineBreaker {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineBreaker.GreedyLineBreaker"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn LineBreaker> {
        Box::new(self.clone())
    }

    fn get_strategy_name(&self) -> UString {
        return UString::from("greedy").to_ustring();
    }

    fn break_lines(&self, n: &Vec<Cluster>, a: &Vec<Cluster>, max_width: f64, shrink_opportunities: Option<Vec<ShrinkOpportunity>>, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters: Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: Option<SortedSetTable<u32>>, cjk_inter_char_boundaries: Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>, line_adjustment_push_in: Option<bool>, line_adjustment_compress_bias: Option<f64>, hard_break_after_clusters: Option<SortedSetTable<u32>>, non_rendering_control_clusters: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        if u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
        }
        if u32::try_from((n.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: UString::from("naturalClusters and adjustedClusters must align cluster-for-cluster.") });
        }
        let shrink_ops = match &(shrink_opportunities) { None => vec![], Some(__option20) => (*__option20).clone() };
        let ranges = match &(unbreakable_ranges) { None => (*crate::org::tiqian::layout::progressive_break_decisions::UNBREAKABLE_RANGES_EMPTY).clone(), Some(__option21) => (*__option21).clone() };
        let indent = match &(first_line_indent) { None => 0 as f64, Some(__option22) => *__option22 };
        let hangables = match &(hangable_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option23) => (*__option23).clone() };
        let extendables = match &(extendable_hang_ranges) { None => vec![], Some(__option24) => (*__option24).clone() };
        let forbid_end = match &(forbidden_line_end_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option25) => (*__option25).clone() };
        let hyphens = match &(hyphen_break_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option26) => (*__option26).clone() };
        let cjk = match &(cjk_inter_char_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option27) => (*__option27).clone() };
        let max_stretch = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option28) => *__option28 };
        let sino = match &(sino_western_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option29) => (*__option29).clone() };
        let sino_cap = match &(sino_western_stretch_cap) { None => 0 as f64, Some(__option30) => *__option30 };
        let hard = match &(hard_break_after_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option31) => (*__option31).clone() };
        let controls = match &(non_rendering_control_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option32) => (*__option32).clone() };
        let progressive = match &(progressive_break_opportunities) { None => LineBreakerLines::line_breaker_lines_empty_progressive_map(), Some(__option33) => (*__option33).clone() };
        let greedy = self.greedy_fill(&n, &a, max_width, (ranges).clone(), indent, (forbid_end).clone(), (hyphens).clone(), (cjk).clone(), max_stretch, (sino).clone(), sino_cap, (hard).clone(), (controls).clone(), (progressive).clone())?;
        let repaired = LineRepair::line_repair_apply_kinsoku_repairs(&greedy, &n, &a, max_width, (self.kinsoku).clone(), &shrink_ops, self.push_in_penalty, self.carry_previous_penalty, self.leave_ragged_penalty, Some((ranges).clone()), Some(indent), Some((hangables).clone()), Some((extendables).clone()), Some(5), (forbidden_line_start_clusters).clone())?;
        let mut gap_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut gi = 0u32;
        while (i32::from_ne_bytes(((gi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((cjk.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(cjk.at(i32::from_ne_bytes(((gi) as i32).to_ne_bytes()))));
            gi = u32::wrapping_add(gi, 1);
        }
        let mut si = 0u32;
        while (i32::from_ne_bytes(((si) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((sino.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(sino.at(i32::from_ne_bytes(((si) as i32).to_ne_bytes()))));
            si = u32::wrapping_add(si, 1);
        }
        let gap_boundaries: SortedSetTable<u32> = gap_builder.clone().build();
        let push_in = match &(line_adjustment_push_in) { None => false, Some(__option34) => *__option34 };
        let compress_bias = match &(line_adjustment_compress_bias) { None => 1.0f64, Some(__option35) => *__option35 };
        return Ok(LineRepair::line_repair_with_fill_push_in((repaired).clone(), push_in, &n, &a, max_width, &shrink_ops, indent, compress_bias, (forbidden_line_start_clusters).clone(), (forbid_end).clone(), (ranges).clone(), self.push_in_penalty, Some((gap_boundaries).clone()), Some((progressive).clone()))?);
    }
}

#[derive(Clone)]
pub struct LookaheadLineBreaker {
    pub(crate) window: u32,
    pub(crate) future_line_horizon: u32,
    pub(crate) raggedness_weight: f64,
    pub(crate) kinsoku: Box<dyn KinsokuRule>,
    pub(crate) push_in_penalty: u32,
    pub(crate) carry_previous_penalty: u32,
    pub(crate) leave_ragged_penalty: u32,
    pub(crate) consecutive_synthetic_hyphen_penalty: f64,
}

impl LookaheadLineBreaker {
    pub fn new(window: Option<u32>, future_line_horizon: Option<u32>, raggedness_weight: Option<f64>, kinsoku: Option<Box<dyn KinsokuRule>>, push_in_penalty: Option<u32>, carry_previous_penalty: Option<u32>, leave_ragged_penalty: Option<u32>, consecutive_synthetic_hyphen_penalty: Option<f64>) -> Self {
        let window = window.unwrap_or_else(|| 2);
        let future_line_horizon = future_line_horizon.unwrap_or_else(|| 2);
        let raggedness_weight = raggedness_weight.unwrap_or_else(|| 0.5);
        let kinsoku = kinsoku.unwrap_or_else(|| Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic))));
        let push_in_penalty = push_in_penalty.unwrap_or_else(|| 2);
        let carry_previous_penalty = carry_previous_penalty.unwrap_or_else(|| 10);
        let leave_ragged_penalty = leave_ragged_penalty.unwrap_or_else(|| 20);
        let consecutive_synthetic_hyphen_penalty = consecutive_synthetic_hyphen_penalty.unwrap_or_else(|| 12.0);
        Self {
            window: window,
            future_line_horizon: future_line_horizon,
            raggedness_weight: raggedness_weight,
            kinsoku: kinsoku,
            push_in_penalty: push_in_penalty,
            carry_previous_penalty: carry_previous_penalty,
            leave_ragged_penalty: leave_ragged_penalty,
            consecutive_synthetic_hyphen_penalty: consecutive_synthetic_hyphen_penalty,
        }
    }

    pub fn get_strategy_name(&self) -> UString {
        return UString::from("lookahead").to_ustring();
    }

    pub fn break_lines(&self, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: Option<Vec<ShrinkOpportunity>>, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters: Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: Option<SortedSetTable<u32>>, cjk_inter_char_boundaries: Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>, line_adjustment_push_in: Option<bool>, line_adjustment_compress_bias: Option<f64>, hard_break_after_clusters: Option<SortedSetTable<u32>>, non_rendering_control_clusters: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        if u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
        }
        if u32::try_from((natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: UString::from("naturalClusters and adjustedClusters must align cluster-for-cluster.") });
        }
        if self.window > 2147483647 {
            return Err(TextRangeError::Message { text: UString::from("window must be non-negative.") });
        }
        if self.future_line_horizon > 2147483647 {
            return Err(TextRangeError::Message { text: UString::from("futureLineHorizon must be non-negative.") });
        }
        let shrink_ops = match &(shrink_opportunities) { None => vec![], Some(__option36) => (*__option36).clone() };
        let ranges = match &(unbreakable_ranges) { None => (*crate::org::tiqian::layout::progressive_break_decisions::UNBREAKABLE_RANGES_EMPTY).clone(), Some(__option37) => (*__option37).clone() };
        let indent = match &(first_line_indent) { None => 0.0f64, Some(__option38) => *__option38 };
        let hangables = match &(hangable_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option39) => (*__option39).clone() };
        let extendables = match &(extendable_hang_ranges) { None => vec![], Some(__option40) => (*__option40).clone() };
        let forbid_end = match &(forbidden_line_end_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option41) => (*__option41).clone() };
        let hyphens = match &(hyphen_break_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option42) => (*__option42).clone() };
        let cjk = match &(cjk_inter_char_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option43) => (*__option43).clone() };
        let max_stretch = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option44) => *__option44 };
        let sino = match &(sino_western_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option45) => (*__option45).clone() };
        let sino_cap = match &(sino_western_stretch_cap) { None => 0.0f64, Some(__option46) => *__option46 };
        let push_in = match &(line_adjustment_push_in) { None => false, Some(__option47) => *__option47 };
        let compress_bias = match &(line_adjustment_compress_bias) { None => 1.0f64, Some(__option48) => *__option48 };
        let hard = match &(hard_break_after_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option49) => (*__option49).clone() };
        let controls = match &(non_rendering_control_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option50) => (*__option50).clone() };
        let progressive = match &(progressive_break_opportunities) { None => LineBreakerLines::line_breaker_lines_empty_progressive_map(), Some(__option51) => (*__option51).clone() };
        let mut committed: Vec<LineCandidate> = vec![];
        let mut line_start = 0u32;
        let mut gap_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut gi = 0u32;
        while (i32::from_ne_bytes(((gi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((cjk.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(cjk.at(i32::from_ne_bytes(((gi) as i32).to_ne_bytes()))));
            gi = u32::wrapping_add(gi, 1);
        }
        let mut si = 0u32;
        while (i32::from_ne_bytes(((si) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((sino.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(sino.at(i32::from_ne_bytes(((si) as i32).to_ne_bytes()))));
            si = u32::wrapping_add(si, 1);
        }
        let gap_boundaries: SortedSetTable<u32> = gap_builder.clone().build();
        let d_ref = max_stretch;
        let mut committed_density = 0.0f64;
        let mut committed_synthetic_hyphen_run = 0u32;
        let mut sorted_breaks: Vec<u32> = vec![];
        let mut bi = 0u32;
        while (i32::from_ne_bytes(((bi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((hard.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            sorted_breaks.push(hard.at(i32::from_ne_bytes(((bi) as i32).to_ne_bytes())));
            bi = u32::wrapping_add(bi, 1);
        }
        let mut break_cursor = 0u32;
        while (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            while (i32::from_ne_bytes(((break_cursor) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && ({ let v: u32 = sorted_breaks[usize::try_from(break_cursor).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) {
                break_cursor = u32::wrapping_add(break_cursor, 1);
            }
            let mandatory_end = if i32::from_ne_bytes(((break_cursor) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { Some(sorted_breaks[usize::try_from(break_cursor).unwrap_or(0)]) } else { None };
            let segment_end_exclusive = match &(mandatory_end) { Some(__option52) => u32::wrapping_add(*__option52, 1), None => u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) };
            let raw_greedy_end = LineBreakerLines::line_breaker_lines_find_greedy_end(&adjusted_clusters, line_start, ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start), Some(segment_end_exclusive), Some((controls).clone()));
            let prog_break = ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(line_start, raw_greedy_end, (progressive).clone(), Some((adjusted_clusters).clone()), Some(ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start)), Some((cjk).clone()), Some(max_stretch), Some((sino).clone()), Some(sino_cap));
            let hyphen_break = ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(line_start, prog_break, &adjusted_clusters, ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start), (hyphens).clone(), (cjk).clone(), max_stretch, Some((sino).clone()), Some(sino_cap));
            let greedy_end = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(hyphen_break, line_start, (ranges).clone());
            if i32::from_ne_bytes(((greedy_end) as i32).to_ne_bytes()) >= { let v: u32 = segment_end_exclusive; i32::from_ne_bytes(v.to_ne_bytes()) } {
                match &(mandatory_end) {
                    Some(__option53) => {
                        committed.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, *__option53), &natural_clusters, &adjusted_clusters, Some(LineEndReason::MandatoryBreak), None, None)?);
                        committed_density = 0.0f64;
                        committed_synthetic_hyphen_run = 0u32;
                        line_start = u32::wrapping_add(*__option53, 1);
                        if line_start == u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
                            committed.push(LineBreakerLines::line_breaker_lines_empty_line_candidate(((adjusted_clusters[usize::try_from(u32::wrapping_sub(u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end, Some(LineEndReason::ParagraphEnd))?);
                        }
                        continue;
                    }
                    None => {
                    }
                }
                committed.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, u32::wrapping_sub(u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)), &natural_clusters, &adjusted_clusters, Some(LineEndReason::ParagraphEnd), None, None)?);
                break;
            }
            let mut candidates: Vec<u32> = vec![];
            let cand_start = u32::wrapping_sub(greedy_end, self.window);
            let mut c = cand_start;
            while (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((greedy_end) as i32).to_ne_bytes()) {
                if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((u32::wrapping_add(line_start, 1)) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= { let v: u32 = segment_end_exclusive; i32::from_ne_bytes(v.to_ne_bytes()) } && !ranges.contains_boundary(c) && ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(line_start, raw_greedy_end, c, (progressive).clone(), Some((adjusted_clusters).clone()), Some(ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start)), Some((cjk).clone()), Some(max_stretch), Some((sino).clone()), Some(sino_cap)) && (LookaheadLineBreaker::lookahead_line_breaker_has_rendering_content_in_range(line_start, c, (controls).clone()) || c == segment_end_exclusive) {
                    if u32::from_ne_bytes(((match candidates.iter().position(|e| e == &c) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) == 4294967295u32 {
                        candidates.push(c);
                    }
                }
                c = u32::wrapping_add(c, 1);
            }
            if u32::try_from((candidates.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
                candidates.push(ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(greedy_end, line_start, (ranges).clone()));
            }
            let mut best_end = greedy_end;
            let mut best_score = f64::INFINITY;
            for &e in &candidates {
                let score = self.score_candidate(line_start, e, &natural_clusters, &adjusted_clusters, max_width, &shrink_ops, indent, (hangables).clone(), &extendables, (forbidden_line_start_clusters).clone(), (hyphens).clone(), (cjk).clone(), max_stretch, (sino).clone(), sino_cap, segment_end_exclusive, committed_density, committed_synthetic_hyphen_run, (gap_boundaries).clone(), d_ref, (ranges).clone(), (controls).clone(), (progressive).clone())?;
                if score < (best_score) {
                    best_score = score;
                    best_end = e;
                }
            }
            let committed_end = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_line_end(best_end, line_start, (forbid_end).clone());
            if hard.has(&(committed_end)) && (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((committed_end) as i32).to_ne_bytes())) {
                committed.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, committed_end), &natural_clusters, &adjusted_clusters, Some(LineEndReason::MandatoryBreak), None, None)?);
                committed_density = 0.0f64;
                committed_synthetic_hyphen_run = 0u32;
                line_start = u32::wrapping_add(committed_end, 1);
                if line_start == u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
                    committed.push(LineBreakerLines::line_breaker_lines_empty_line_candidate(((adjusted_clusters[usize::try_from(u32::wrapping_sub(u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end, Some(LineEndReason::ParagraphEnd))?);
                }
                continue;
            }
            committed.push(LineBreakerLines::line_breaker_lines_close_filled_line(IntRange::new(line_start, u32::wrapping_sub(committed_end, 1)), best_end, &natural_clusters, &adjusted_clusters)?);
            let last_line = (committed[usize::try_from(u32::wrapping_sub(u32::try_from((committed.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
            let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, last_line.cluster_range.start);
            committed_density = LineBreakerLines::line_breaker_lines_line_adjustment_density((last_line).clone(), limit, false, (gap_boundaries).clone());
            committed_synthetic_hyphen_run = if LineBreakerLines::line_breaker_lines_ends_with_synthetic_hyphen((last_line).clone(), (hyphens).clone()) { u32::wrapping_add(committed_synthetic_hyphen_run, 1) } else { 0 };
            line_start = committed_end;
        }
        let repaired = LineRepair::line_repair_apply_kinsoku_repairs(&committed, &natural_clusters, &adjusted_clusters, max_width, (self.kinsoku).clone(), &shrink_ops, self.push_in_penalty, self.carry_previous_penalty, self.leave_ragged_penalty, Some((ranges).clone()), Some(indent), Some((hangables).clone()), Some((extendables).clone()), Some(5), (forbidden_line_start_clusters).clone())?;
        return Ok(LineRepair::line_repair_with_fill_push_in((repaired).clone(), push_in, &natural_clusters, &adjusted_clusters, max_width, &shrink_ops, indent, compress_bias, (forbidden_line_start_clusters).clone(), (forbid_end).clone(), (ranges).clone(), self.push_in_penalty, Some((gap_boundaries).clone()), Some((progressive).clone()))?);
    }

    fn score_candidate(&self, s: u32, e: u32, natural: &Vec<Cluster>, adjusted: &Vec<Cluster>, max_width: f64, shrink_opportunities: &Vec<ShrinkOpportunity>, first_line_indent: f64, hangable_clusters: SortedSetTable<u32>, extendable_hang_ranges: &Vec<IntRange>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: SortedSetTable<u32>, cjk_inter_char_boundaries: SortedSetTable<u32>, max_cjk_stretch_per_gap: f64, sino_western_boundaries: SortedSetTable<u32>, sino_western_stretch_cap: f64, segment_end_exclusive: u32, prev_committed_density: f64, prev_synthetic_hyphen_run: u32, gap_boundaries: SortedSetTable<u32>, d_ref: f64, unbreakable_ranges: UnbreakableRanges, non_rendering_control_clusters: SortedSetTable<u32>, progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>) -> Result<f64, TextRangeError> {
        let first_line = LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(s, u32::wrapping_sub(e, 1)), &natural, &adjusted, None, None, None)?;
        let future = self.raw_greedy_lines_from(e, &natural, &adjusted, max_width, (hyphen_break_clusters).clone(), (cjk_inter_char_boundaries).clone(), max_cjk_stretch_per_gap, (sino_western_boundaries).clone(), sino_western_stretch_cap, segment_end_exclusive, (unbreakable_ranges).clone(), (non_rendering_control_clusters).clone(), u32::wrapping_add(self.future_line_horizon, 1), (progressive_break_opportunities).clone())?;
        let spliced = (LineRepair::line_repair_apply_kinsoku_repairs(&{ let mut result = vec![(first_line).clone()].clone(); result.extend(future.iter().cloned()); result }, &natural, &adjusted, max_width, (self.kinsoku).clone(), &shrink_opportunities, self.push_in_penalty, self.carry_previous_penalty, self.leave_ragged_penalty, Some((unbreakable_ranges).clone()), Some(first_line_indent), Some((hangable_clusters).clone()), Some((extendable_hang_ranges).clone()), Some(5), (forbidden_line_start_clusters).clone())?.lines).clone();
        let horizon = u32::from_ne_bytes(((match f64::from({ let __min_a1 = i32::from_ne_bytes(((u32::wrapping_add(1, self.future_line_horizon)) as i32).to_ne_bytes()) as f64 as f64; let __min_b1 = i32::from_ne_bytes(((u32::try_from((spliced.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) as f64 as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 < __min_b1 { __min_a1 } else if __min_b1 < __min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_a1 } else { __min_b1 } } else { __min_a1 } } }) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes());
        let mut score = 0.0f64;
        let mut prev_d = prev_committed_density;
        let mut synthetic_hyphen_run = prev_synthetic_hyphen_run;
        let mut idx = 0u32;
        while (i32::from_ne_bytes(((idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((horizon) as i32).to_ne_bytes())) {
            let line = (spliced[usize::try_from(idx).unwrap_or(0)]).clone();
            let is_last = idx == u32::wrapping_sub(u32::try_from((spliced.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
            score += self.badness((line).clone(), max_width, is_last, first_line_indent, prev_d, (gap_boundaries).clone(), d_ref);
            if LineBreakerLines::line_breaker_lines_ends_with_synthetic_hyphen((line).clone(), (hyphen_break_clusters).clone()) {
                score += self.consecutive_synthetic_hyphen_penalty * format!("{}", (i32::from_ne_bytes(((synthetic_hyphen_run) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0);
                synthetic_hyphen_run = u32::wrapping_add(synthetic_hyphen_run, 1);
            } else {
                synthetic_hyphen_run = 0u32;
            }
            let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, first_line_indent, line.cluster_range.start);
            prev_d = LineBreakerLines::line_breaker_lines_line_adjustment_density((line).clone(), limit, is_last, (gap_boundaries).clone());
            idx = u32::wrapping_add(idx, 1);
        }
        return Ok(score);
    }

    fn raw_greedy_lines_from(&self, start: u32, natural: &Vec<Cluster>, adjusted: &Vec<Cluster>, max_width: f64, hyphen_break_clusters: SortedSetTable<u32>, cjk_inter_char_boundaries: SortedSetTable<u32>, max_cjk_stretch_per_gap: f64, sino_western_boundaries: SortedSetTable<u32>, sino_western_stretch_cap: f64, end_exclusive: u32, unbreakable_ranges: UnbreakableRanges, non_rendering_control_clusters: SortedSetTable<u32>, max_lines: u32, progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>) -> Result<Vec<LineCandidate>, TextRangeError> {
        if i32::from_ne_bytes(((start) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((end_exclusive) as i32).to_ne_bytes()) {
            return Ok(vec![]);
        }
        if i32::from_ne_bytes(((max_lines) as i32).to_ne_bytes()) <= 0 {
            return Err(TextRangeError::Message { text: UString::from("maxLines must be positive") });
        }
        let mut lines: Vec<LineCandidate> = vec![];
        let mut line_start = start;
        let mut adjusted_accum = 0.0f64;
        let mut has_rendering_content = false;
        let mut i = start;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end_exclusive) as i32).to_ne_bytes())) {
            let next_adjusted = adjusted_accum + adjusted[usize::try_from(i).unwrap_or(0)].advance;
            let overflows = next_adjusted > (max_width) && has_rendering_content;
            if overflows {
                let prog = ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(line_start, i, (progressive_break_opportunities).clone(), Some((adjusted).clone()), Some(max_width), Some((cjk_inter_char_boundaries).clone()), Some(max_cjk_stretch_per_gap), Some((sino_western_boundaries).clone()), Some(sino_western_stretch_cap));
                let decided = ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(line_start, prog, &adjusted, max_width, (hyphen_break_clusters).clone(), (cjk_inter_char_boundaries).clone(), max_cjk_stretch_per_gap, Some((sino_western_boundaries).clone()), Some(sino_western_stretch_cap));
                let break_at = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(decided, line_start, (unbreakable_ranges).clone());
                lines.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, u32::wrapping_sub(break_at, 1)), &natural, &adjusted, None, None, None)?);
                if i32::from_ne_bytes(((u32::try_from((lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((max_lines) as i32).to_ne_bytes()) {
                    return Ok(lines);
                }
                line_start = break_at;
                adjusted_accum = adjusted[usize::try_from(break_at).unwrap_or(0)].advance;
                has_rendering_content = !non_rendering_control_clusters.has(&(break_at));
                i = u32::wrapping_add(break_at, 1);
            } else {
                adjusted_accum = next_adjusted;
                if !non_rendering_control_clusters.has(&(i)) {
                    has_rendering_content = true;
                }
                i = u32::wrapping_add(i, 1);
            }
        }
        lines.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, u32::wrapping_sub(end_exclusive, 1)), &natural, &adjusted, Some(LineEndReason::ParagraphEnd), None, None)?);
        return Ok(lines);
    }

    fn badness(&self, line: LineCandidate, max_width: f64, is_last: bool, first_line_indent: f64, prev_density: f64, gap_boundaries: SortedSetTable<u32>, d_ref: f64) -> f64 {
        let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, first_line_indent, line.cluster_range.start);
        let ragged = if is_last { 0.0f64 } else { { let __min_a2 = 0.0f64 as f64; let __min_b2 = (limit - line.adjusted_width) as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 == 0.0 && __min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } } };
        let in_measure_range = line.get_in_measure_cluster_range();
        let gaps = LineBreakerLines::line_breaker_lines_line_gap_count((in_measure_range).clone(), (gap_boundaries).clone());
        let residual = if gaps == 0 { ragged } else { 0.0f64 };
        let d = LineBreakerLines::line_breaker_lines_line_adjustment_density((line).clone(), limit, is_last, (gap_boundaries).clone());
        let orphan = if !is_last && !in_measure_range.get_is_empty() && in_measure_range.start == in_measure_range.end { format!("{}", (i32::from_ne_bytes(((self.leave_ragged_penalty) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * 1.0f64 } else { 0.0f64 };
        let repair_penalty = match &(line.repair) { Some(__option54) => format!("{}", (i32::from_ne_bytes(((RepairOptions::repair_options_penalty((*__option54).clone())) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * 1.0f64, None => 0.0f64 };
        return residual * self.raggedness_weight + orphan + LineBreakerLines::line_breaker_lines_amortized_adjustment_cost(d, prev_density, d_ref) * self.raggedness_weight + repair_penalty;
    }

    pub(crate) fn lookahead_line_breaker_has_rendering_content_in_range(start: u32, end: u32, controls: SortedSetTable<u32>) -> bool {
        let mut i = start;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) {
            if !controls.has(&(i)) {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }
}

impl LineBreaker for LookaheadLineBreaker {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineBreaker.LookaheadLineBreaker"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn LineBreaker> {
        Box::new(self.clone())
    }

    fn get_strategy_name(&self) -> UString {
        return UString::from("lookahead").to_ustring();
    }

    fn break_lines(&self, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: Option<Vec<ShrinkOpportunity>>, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters: Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: Option<SortedSetTable<u32>>, cjk_inter_char_boundaries: Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>, line_adjustment_push_in: Option<bool>, line_adjustment_compress_bias: Option<f64>, hard_break_after_clusters: Option<SortedSetTable<u32>>, non_rendering_control_clusters: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        if u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
        }
        if u32::try_from((natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: UString::from("naturalClusters and adjustedClusters must align cluster-for-cluster.") });
        }
        if self.window > 2147483647 {
            return Err(TextRangeError::Message { text: UString::from("window must be non-negative.") });
        }
        if self.future_line_horizon > 2147483647 {
            return Err(TextRangeError::Message { text: UString::from("futureLineHorizon must be non-negative.") });
        }
        let shrink_ops = match &(shrink_opportunities) { None => vec![], Some(__option55) => (*__option55).clone() };
        let ranges = match &(unbreakable_ranges) { None => (*crate::org::tiqian::layout::progressive_break_decisions::UNBREAKABLE_RANGES_EMPTY).clone(), Some(__option56) => (*__option56).clone() };
        let indent = match &(first_line_indent) { None => 0.0f64, Some(__option57) => *__option57 };
        let hangables = match &(hangable_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option58) => (*__option58).clone() };
        let extendables = match &(extendable_hang_ranges) { None => vec![], Some(__option59) => (*__option59).clone() };
        let forbid_end = match &(forbidden_line_end_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option60) => (*__option60).clone() };
        let hyphens = match &(hyphen_break_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option61) => (*__option61).clone() };
        let cjk = match &(cjk_inter_char_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option62) => (*__option62).clone() };
        let max_stretch = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option63) => *__option63 };
        let sino = match &(sino_western_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option64) => (*__option64).clone() };
        let sino_cap = match &(sino_western_stretch_cap) { None => 0.0f64, Some(__option65) => *__option65 };
        let push_in = match &(line_adjustment_push_in) { None => false, Some(__option66) => *__option66 };
        let compress_bias = match &(line_adjustment_compress_bias) { None => 1.0f64, Some(__option67) => *__option67 };
        let hard = match &(hard_break_after_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option68) => (*__option68).clone() };
        let controls = match &(non_rendering_control_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option69) => (*__option69).clone() };
        let progressive = match &(progressive_break_opportunities) { None => LineBreakerLines::line_breaker_lines_empty_progressive_map(), Some(__option70) => (*__option70).clone() };
        let mut committed: Vec<LineCandidate> = vec![];
        let mut line_start = 0u32;
        let mut gap_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut gi = 0u32;
        while (i32::from_ne_bytes(((gi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((cjk.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(cjk.at(i32::from_ne_bytes(((gi) as i32).to_ne_bytes()))));
            gi = u32::wrapping_add(gi, 1);
        }
        let mut si = 0u32;
        while (i32::from_ne_bytes(((si) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((sino.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            gap_builder.put(&(sino.at(i32::from_ne_bytes(((si) as i32).to_ne_bytes()))));
            si = u32::wrapping_add(si, 1);
        }
        let gap_boundaries: SortedSetTable<u32> = gap_builder.clone().build();
        let d_ref = max_stretch;
        let mut committed_density = 0.0f64;
        let mut committed_synthetic_hyphen_run = 0u32;
        let mut sorted_breaks: Vec<u32> = vec![];
        let mut bi = 0u32;
        while (i32::from_ne_bytes(((bi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((hard.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            sorted_breaks.push(hard.at(i32::from_ne_bytes(((bi) as i32).to_ne_bytes())));
            bi = u32::wrapping_add(bi, 1);
        }
        let mut break_cursor = 0u32;
        while (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            while (i32::from_ne_bytes(((break_cursor) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && ({ let v: u32 = sorted_breaks[usize::try_from(break_cursor).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) {
                break_cursor = u32::wrapping_add(break_cursor, 1);
            }
            let mandatory_end = if i32::from_ne_bytes(((break_cursor) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { Some(sorted_breaks[usize::try_from(break_cursor).unwrap_or(0)]) } else { None };
            let segment_end_exclusive = match &(mandatory_end) { Some(__option71) => u32::wrapping_add(*__option71, 1), None => u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) };
            let raw_greedy_end = LineBreakerLines::line_breaker_lines_find_greedy_end(&adjusted_clusters, line_start, ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start), Some(segment_end_exclusive), Some((controls).clone()));
            let prog_break = ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(line_start, raw_greedy_end, (progressive).clone(), Some((adjusted_clusters).clone()), Some(ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start)), Some((cjk).clone()), Some(max_stretch), Some((sino).clone()), Some(sino_cap));
            let hyphen_break = ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(line_start, prog_break, &adjusted_clusters, ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start), (hyphens).clone(), (cjk).clone(), max_stretch, Some((sino).clone()), Some(sino_cap));
            let greedy_end = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(hyphen_break, line_start, (ranges).clone());
            if i32::from_ne_bytes(((greedy_end) as i32).to_ne_bytes()) >= { let v: u32 = segment_end_exclusive; i32::from_ne_bytes(v.to_ne_bytes()) } {
                match &(mandatory_end) {
                    Some(__option72) => {
                        committed.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, *__option72), &natural_clusters, &adjusted_clusters, Some(LineEndReason::MandatoryBreak), None, None)?);
                        committed_density = 0.0f64;
                        committed_synthetic_hyphen_run = 0u32;
                        line_start = u32::wrapping_add(*__option72, 1);
                        if line_start == u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
                            committed.push(LineBreakerLines::line_breaker_lines_empty_line_candidate(((adjusted_clusters[usize::try_from(u32::wrapping_sub(u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end, Some(LineEndReason::ParagraphEnd))?);
                        }
                        continue;
                    }
                    None => {
                    }
                }
                committed.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, u32::wrapping_sub(u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)), &natural_clusters, &adjusted_clusters, Some(LineEndReason::ParagraphEnd), None, None)?);
                break;
            }
            let mut candidates: Vec<u32> = vec![];
            let cand_start = u32::wrapping_sub(greedy_end, self.window);
            let mut c = cand_start;
            while (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((greedy_end) as i32).to_ne_bytes()) {
                if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((u32::wrapping_add(line_start, 1)) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= { let v: u32 = segment_end_exclusive; i32::from_ne_bytes(v.to_ne_bytes()) } && !ranges.contains_boundary(c) && ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(line_start, raw_greedy_end, c, (progressive).clone(), Some((adjusted_clusters).clone()), Some(ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, line_start)), Some((cjk).clone()), Some(max_stretch), Some((sino).clone()), Some(sino_cap)) && (LookaheadLineBreaker::lookahead_line_breaker_has_rendering_content_in_range(line_start, c, (controls).clone()) || c == segment_end_exclusive) {
                    if u32::from_ne_bytes(((match candidates.iter().position(|e| e == &c) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) == 4294967295u32 {
                        candidates.push(c);
                    }
                }
                c = u32::wrapping_add(c, 1);
            }
            if u32::try_from((candidates.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
                candidates.push(ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(greedy_end, line_start, (ranges).clone()));
            }
            let mut best_end = greedy_end;
            let mut best_score = f64::INFINITY;
            for &e in &candidates {
                let score = self.score_candidate(line_start, e, &natural_clusters, &adjusted_clusters, max_width, &shrink_ops, indent, (hangables).clone(), &extendables, (forbidden_line_start_clusters).clone(), (hyphens).clone(), (cjk).clone(), max_stretch, (sino).clone(), sino_cap, segment_end_exclusive, committed_density, committed_synthetic_hyphen_run, (gap_boundaries).clone(), d_ref, (ranges).clone(), (controls).clone(), (progressive).clone())?;
                if score < (best_score) {
                    best_score = score;
                    best_end = e;
                }
            }
            let committed_end = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_line_end(best_end, line_start, (forbid_end).clone());
            if hard.has(&(committed_end)) && (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((committed_end) as i32).to_ne_bytes())) {
                committed.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, committed_end), &natural_clusters, &adjusted_clusters, Some(LineEndReason::MandatoryBreak), None, None)?);
                committed_density = 0.0f64;
                committed_synthetic_hyphen_run = 0u32;
                line_start = u32::wrapping_add(committed_end, 1);
                if line_start == u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
                    committed.push(LineBreakerLines::line_breaker_lines_empty_line_candidate(((adjusted_clusters[usize::try_from(u32::wrapping_sub(u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end, Some(LineEndReason::ParagraphEnd))?);
                }
                continue;
            }
            committed.push(LineBreakerLines::line_breaker_lines_close_filled_line(IntRange::new(line_start, u32::wrapping_sub(committed_end, 1)), best_end, &natural_clusters, &adjusted_clusters)?);
            let last_line = (committed[usize::try_from(u32::wrapping_sub(u32::try_from((committed.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
            let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, last_line.cluster_range.start);
            committed_density = LineBreakerLines::line_breaker_lines_line_adjustment_density((last_line).clone(), limit, false, (gap_boundaries).clone());
            committed_synthetic_hyphen_run = if LineBreakerLines::line_breaker_lines_ends_with_synthetic_hyphen((last_line).clone(), (hyphens).clone()) { u32::wrapping_add(committed_synthetic_hyphen_run, 1) } else { 0 };
            line_start = committed_end;
        }
        let repaired = LineRepair::line_repair_apply_kinsoku_repairs(&committed, &natural_clusters, &adjusted_clusters, max_width, (self.kinsoku).clone(), &shrink_ops, self.push_in_penalty, self.carry_previous_penalty, self.leave_ragged_penalty, Some((ranges).clone()), Some(indent), Some((hangables).clone()), Some((extendables).clone()), Some(5), (forbidden_line_start_clusters).clone())?;
        return Ok(LineRepair::line_repair_with_fill_push_in((repaired).clone(), push_in, &natural_clusters, &adjusted_clusters, max_width, &shrink_ops, indent, compress_bias, (forbidden_line_start_clusters).clone(), (forbid_end).clone(), (ranges).clone(), self.push_in_penalty, Some((gap_boundaries).clone()), Some((progressive).clone()))?);
    }
}

#[derive(Clone, Copy)]
pub struct LineBreakerLines;

impl LineBreakerLines {
    pub fn line_breaker_lines_close_filled_line(range: IntRange, natural_break_at: u32, n: &Vec<Cluster>, a: &Vec<Cluster>) -> Result<LineCandidate, TextRangeError> {
        let line = LineBreakerLines::line_breaker_lines_rebuild_line((range).clone(), &n, &a, None, None, None)?;
        if u32::wrapping_add(range.end, 1) == natural_break_at {
            return Ok(line);
        }
        let moved = u32::wrapping_add(range.end, 1);
        return Ok(LineCandidate::new((line.cluster_range).clone(), (line.source_range).clone(), line.natural_width, line.adjusted_width, Some(line.end_reason), Some(RepairOption::CarryNext { penalty: 0, reason: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ForbiddenAtLineEnd:")); __s += ((a[usize::try_from(moved).unwrap_or(0)]).clone().text).to_ustring().as_ustr(); __s += &(UString::from(":moved-to-next-line")); __s }).as_str()), moved_cluster_index: moved }), Some((line.repair_candidates).clone()), Some((line.hanging_cluster_indices).clone()))?);
    }

    pub fn line_breaker_lines_rebuild_line(cluster_range: IntRange, n: &Vec<Cluster>, a: &Vec<Cluster>, end_reason: Option<LineEndReason>, repair: Option<RepairOption>, repair_candidates: Option<Vec<RepairCandidate>>) -> Result<LineCandidate, TextRangeError> {
        if cluster_range.get_is_empty() {
            return Err(TextRangeError::Message { text: UString::from("Use emptyLineCandidate for an empty line.") });
        }
        let mut natural = 0.0f64;
        let mut adjusted = 0.0f64;
        let mut idx = cluster_range.start;
        while (i32::from_ne_bytes(((idx) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((cluster_range.end) as i32).to_ne_bytes()) {
            natural += n[usize::try_from(idx).unwrap_or(0)].advance;
            adjusted += a[usize::try_from(idx).unwrap_or(0)].advance;
            idx = u32::wrapping_add(idx, 1);
        }
        return Ok(LineCandidate::new((cluster_range).clone(), TextRange::new(((a[usize::try_from(cluster_range.start).unwrap_or(0)]).clone().range).clone().start, ((a[usize::try_from(cluster_range.end).unwrap_or(0)]).clone().range).clone().end)?, natural, adjusted, end_reason, (repair).clone(), (repair_candidates).clone(), Some(LineCandidate::line_candidate_empty_hanging()))?);
    }

    pub fn line_breaker_lines_empty_line_candidate(source_offset: u32, end_reason: Option<LineEndReason>) -> Result<LineCandidate, TextRangeError> {
        return Ok(LineCandidate::new(IntRange::new(1u32, 0u32), TextRange::new(source_offset, source_offset)?, 0 as f64 as f64, 0 as f64 as f64, end_reason, None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging()))?);
    }

    pub fn line_breaker_lines_empty_int_set() -> SortedSetTable<u32> {
        let b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        return b.clone().build();
    }

    pub fn line_breaker_lines_empty_progressive_map() -> SortedMapTable<u32, ProgressiveBreakOpportunity> {
        let b: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        return b.clone().build();
    }

    pub fn line_breaker_lines_ends_with_synthetic_hyphen(line: LineCandidate, hyphen_break_clusters: SortedSetTable<u32>) -> bool {
        return line.end_reason == LineEndReason::AutoWrap && !line.cluster_range.get_is_empty() && hyphen_break_clusters.has(&(u32::wrapping_add(line.cluster_range.end, 1)));
    }

    pub fn line_breaker_lines_ends_with_progressive_break(candidate: LineCandidate, opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>) -> bool {
        return candidate.end_reason == LineEndReason::AutoWrap && !candidate.cluster_range.get_is_empty() && opportunities.get(&(u32::wrapping_add(candidate.cluster_range.end, 1))).is_some();
    }

    pub fn line_breaker_lines_line_gap_count(range: IntRange, gap_boundaries: SortedSetTable<u32>) -> u32 {
        if range.get_is_empty() {
            return 0;
        }
        let mut n = 0u32;
        let mut i = range.start;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((range.end) as i32).to_ne_bytes())) {
            if gap_boundaries.has(&(i)) {
                n = u32::wrapping_add(n, 1);
            }
            i = u32::wrapping_add(i, 1);
        }
        return n;
    }

    pub fn line_breaker_lines_line_adjustment_density(line: LineCandidate, limit: f64, is_last: bool, gap_boundaries: SortedSetTable<u32>) -> f64 {
        if is_last || line.end_reason != LineEndReason::AutoWrap {
            return 0.0f64;
        }
        let gaps = LineBreakerLines::line_breaker_lines_line_gap_count(line.get_in_measure_cluster_range(), (gap_boundaries).clone());
        if gaps == 0 {
            return 0.0f64;
        }
        let delta = { let __min_a3 = 0.0f64 as f64; let __min_b3 = (limit - line.adjusted_width) as f64; if __min_a3.is_nan() || __min_b3.is_nan() { f64::NAN } else { if __min_a3 > __min_b3 { __min_a3 } else if __min_b3 > __min_a3 { __min_b3 } else if __min_a3 == 0.0 && __min_b3 == 0.0 { if __min_a3.is_sign_negative() { __min_b3 } else { __min_a3 } } else { __min_a3 } } };
        return delta / format!("{}", (i32::from_ne_bytes(((gaps) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0);
    }

    pub fn line_breaker_lines_amortized_adjustment_cost(d: f64, prev_d: f64, d_ref: f64) -> f64 {
        let r#ref = if d_ref < (1.0f64) { 1.0f64 } else { d_ref };
        let diff = d - prev_d;
        return (d * d + diff * diff) / r#ref;
    }

    pub fn line_breaker_lines_find_greedy_end(clusters: &Vec<Cluster>, start: u32, max_width: f64, end_exclusive: Option<u32>, non_rendering_control_clusters: Option<SortedSetTable<u32>>) -> u32 {
        let end = match &(end_exclusive) { None => u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), Some(__option73) => *__option73 };
        let controls = match &(non_rendering_control_clusters) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option74) => (*__option74).clone() };
        let mut accum = 0.0f64;
        let mut i = start;
        let mut has_rendering_content = false;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) {
            let next = accum + clusters[usize::try_from(i).unwrap_or(0)].advance;
            if next > (max_width) && has_rendering_content {
                return i;
            }
            accum = next;
            if !controls.has(&(i)) {
                has_rendering_content = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return end;
    }
}
