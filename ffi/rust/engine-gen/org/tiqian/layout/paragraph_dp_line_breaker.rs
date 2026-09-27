use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::kinsoku_rule::KinsokuRule;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreakerLines;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::line_repair::LineRepair;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use std::sync::Arc;


#[derive(Clone)]
pub struct ParagraphDpLineBreaker {
    pub(crate) candidate_window: u32,
    pub(crate) raggedness_weight: f64,
    pub(crate) kinsoku: Box<dyn KinsokuRule>,
    pub(crate) push_in_penalty: u32,
    pub(crate) carry_previous_penalty: u32,
    pub(crate) leave_ragged_penalty: u32,
    pub(crate) synthetic_hyphen_break_penalty: f64,
    pub(crate) consecutive_synthetic_hyphen_penalty: f64,
    pub(crate) consecutive_stretch_penalty: f64,
    pub(crate) compression_visibility: f64,
}

impl ParagraphDpLineBreaker {
    const PARAGRAPH_DP_LINE_BREAKER_HYPHEN_RUN_STATE_CAP: u32 = 3;

    const PARAGRAPH_DP_LINE_BREAKER_STRETCH_RUN_STATE_CAP: u32 = 3;

    const PARAGRAPH_DP_LINE_BREAKER_VISIBLE_STRETCH_FLOOR_PX: f64 = 0.5f64;

    pub fn new(candidate_window: Option<u32>, raggedness_weight: Option<f64>, kinsoku: Option<Box<dyn KinsokuRule>>, push_in_penalty: Option<u32>, carry_previous_penalty: Option<u32>, leave_ragged_penalty: Option<u32>, synthetic_hyphen_break_penalty: Option<f64>,
consecutive_synthetic_hyphen_penalty: Option<f64>, consecutive_stretch_penalty: Option<f64>, compression_visibility: Option<f64>) -> Result<Self, TextRangeError> {
        let candidate_window = candidate_window.unwrap_or_else(|| 8);
        let raggedness_weight = raggedness_weight.unwrap_or_else(|| 0.5);
        let kinsoku = kinsoku.unwrap_or_else(|| Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic))));
        let push_in_penalty = push_in_penalty.unwrap_or_else(|| 2);
        let carry_previous_penalty = carry_previous_penalty.unwrap_or_else(|| 10);
        let leave_ragged_penalty = leave_ragged_penalty.unwrap_or_else(|| 20);
        let synthetic_hyphen_break_penalty = synthetic_hyphen_break_penalty.unwrap_or_else(|| 12 as f64);
        let consecutive_synthetic_hyphen_penalty = consecutive_synthetic_hyphen_penalty.unwrap_or_else(|| 12 as f64);
        let consecutive_stretch_penalty = consecutive_stretch_penalty.unwrap_or_else(|| 3 as f64);
        let compression_visibility = compression_visibility.unwrap_or_else(|| 1 as f64);
        if candidate_window > 2147483647 {
            return Err(TextRangeError::Message { text: "candidateWindow must be non-negative.".to_string() });
        }
        Ok(Self {
            candidate_window: candidate_window,
            raggedness_weight: raggedness_weight,
            kinsoku: kinsoku,
            push_in_penalty: push_in_penalty,
            carry_previous_penalty: carry_previous_penalty,
            leave_ragged_penalty: leave_ragged_penalty,
            synthetic_hyphen_break_penalty: synthetic_hyphen_break_penalty,
            consecutive_synthetic_hyphen_penalty: consecutive_synthetic_hyphen_penalty,
            consecutive_stretch_penalty: consecutive_stretch_penalty,
            compression_visibility: compression_visibility,
        })
    }

    pub fn get_strategy_name(&self) -> String {
        return "paragraph-dp".to_string();
    }

    pub fn break_lines(&self, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: Option<Vec<ShrinkOpportunity>>, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters:
Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: Option<SortedSetTable<u32>>, cjk_inter_char_boundaries:
Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>, line_adjustment_push_in: Option<bool>, _line_adjustment_compress_bias: Option<f64>, hard_break_after_clusters:
Option<SortedSetTable<u32>>, non_rendering_control_clusters: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        if u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
        }
        if u32::try_from((natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "naturalClusters and adjustedClusters must align cluster-for-cluster.".to_string() });
        }
        let shrink = match &(shrink_opportunities) { None => vec![], Some(__option) => (*__option).clone() };
        let ranges = match &(unbreakable_ranges) { None => UnbreakableRanges::new(vec![].to_vec()), Some(__option1) => (*__option1).clone() };
        let indent = match &(first_line_indent) { None => 0.0f64, Some(__option2) => *__option2 };
        let forbid_start = (forbidden_line_start_clusters).clone();
        let forbid_end = match &(forbidden_line_end_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
Some(__option3) => (*__option3).clone() };
        let hyphens = match &(hyphen_break_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option4) =>
(*__option4).clone() };
        let cjk = match &(cjk_inter_char_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option5) =>
(*__option5).clone() };
        let sino = match &(sino_western_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option6) =>
(*__option6).clone() };
        let mut gap_boundaries_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((cjk.size()).to_ne_bytes()) {
            gap_boundaries_builder.put(&(cjk.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        for i in 0..u32::from_ne_bytes((sino.size()).to_ne_bytes()) {
            gap_boundaries_builder.put(&(sino.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        let gap_boundaries: SortedSetTable<u32> = gap_boundaries_builder.clone().build();
        let controls = match &(non_rendering_control_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
Some(__option7) => (*__option7).clone() };
        let progressive = match &(progressive_break_opportunities) { None => SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option8) => (*__option8).clone() };
        let hard = match &(hard_break_after_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option9)
=> (*__option9).clone() };
        let hangables = match &(hangable_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option10) =>
(*__option10).clone() };
        let max_stretch = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option11) => *__option11 };
        let sino_cap = match &(sino_western_stretch_cap) { None => 0.0f64, Some(__option12) => *__option12 };
        let context = DpContext::new(natural_clusters.to_vec(), adjusted_clusters.to_vec(), max_width, (shrink).clone(), (ranges).clone(), indent, (forbidden_line_start_clusters).clone(), (forbid_end).clone(), (hyphens).clone(), (cjk).clone(), max_stretch, (sino).clone(),
sino_cap, (controls).clone(), (gap_boundaries).clone(), max_stretch, line_adjustment_push_in.as_ref().map_or(false, |v| v == &(true)), (progressive).clone())?;
        let mut committed: Vec<LineCandidate> = vec![];
        let mut sorted_breaks: Vec<u32> = vec![];
        if i32::from_ne_bytes((u32::from_ne_bytes((hard.size()).to_ne_bytes())).to_ne_bytes()) > (0) {
            for i in 0..u32::from_ne_bytes((hard.size()).to_ne_bytes()) {
                sorted_breaks.push(hard.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
            }
        }
        let mut cursor = 0u32;
        let mut segment_start = 0u32;
        while (i32::from_ne_bytes((segment_start).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            while (i32::from_ne_bytes((cursor).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && ({ let v: u32 = sorted_breaks[usize::try_from(cursor).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) <
(i32::from_ne_bytes((segment_start).to_ne_bytes())) {
                cursor = u32::wrapping_add(cursor, 1);
            }
            let mandatory = if i32::from_ne_bytes((cursor).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(sorted_breaks[usize::try_from(cursor).unwrap_or(0)]) } else { None };
            let end = match &(mandatory) { None => u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), Some(__option13) => u32::wrapping_add(*__option13, 1) };
            let ends = self.solve_segment((context).clone(), segment_start, end, mandatory.is_some())?;
            let _ = self.commit_segment(&mut committed, &ends, segment_start, mandatory, (context).clone(), (hard).clone())?;
            segment_start = end;
        }
        return Ok(LineRepair::line_repair_apply_kinsoku_repairs(&committed, &natural_clusters, &adjusted_clusters, max_width, (self.kinsoku).clone(), &shrink, self.push_in_penalty, self.carry_previous_penalty, self.leave_ragged_penalty, Some((ranges).clone()), Some(indent),
Some((hangables).clone()), Some((match &(extendable_hang_ranges) { None => vec![], Some(__option15) => (*__option15).clone() }).clone()), Some(5), (forbid_start).clone())?);
    }

    fn candidate_ends(&self, context: DpContext, start: u32, segment_end_exclusive: u32, ends_with_mandatory: bool) -> Vec<u32> {
        let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(context.max_width, context.first_line_indent, start);
        let raw = LineBreakerLines::line_breaker_lines_find_greedy_end(&context.adjusted_clusters, start, limit, Some(segment_end_exclusive), Some((context.non_rendering_control_clusters).clone()));
        if i32::from_ne_bytes((raw).to_ne_bytes()) >= i32::from_ne_bytes((segment_end_exclusive).to_ne_bytes()) {
            return vec![segment_end_exclusive];
        }
        let progressive = ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(start, raw, (context.progressive_break_opportunities).clone(), Some((context.adjusted_clusters).clone()), Some(limit), Some((context.cjk_inter_char_boundaries).clone()),
Some(context.max_cjk_stretch_per_gap), Some((context.sino_western_boundaries).clone()), Some(context.sino_western_stretch_cap));
        let baseline = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(start, progressive, &context.adjusted_clusters, limit, (context.hyphen_break_clusters).clone(),
(context.cjk_inter_char_boundaries).clone(), context.max_cjk_stretch_per_gap, Some((context.sino_western_boundaries).clone()), Some(context.sino_western_stretch_cap)), start, (context.unbreakable_ranges).clone());
        let mut compressed: Vec<u32> = vec![];
        if context.allow_compression_edges {
            let mut width = 0.0f64;
            let mut i = start;
            while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((raw).to_ne_bytes())) {
                width += context.adjusted_clusters[usize::try_from(i).unwrap_or(0)].advance;
                i = u32::wrapping_add(i, 1);
            }
            let mut e = u32::wrapping_add(raw, 1);
            while (i32::from_ne_bytes((e).to_ne_bytes())) <= i32::from_ne_bytes((segment_end_exclusive).to_ne_bytes()) && (i32::from_ne_bytes((u32::try_from((compressed.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) <
(i32::from_ne_bytes((self.candidate_window).to_ne_bytes())) {
                width += context.adjusted_clusters[usize::try_from(u32::wrapping_sub(e, 1)).unwrap_or(0)].advance;
                if width - limit > (context.shrink_capacity(IntRange::new(start, u32::wrapping_sub(e, 1)))) {
                    break;
                }
                compressed.push(e);
                e = u32::wrapping_add(e, 1);
            }
        }
        let is_promotion: Arc<dyn Fn(u32) -> bool + Send + Sync + 'static> = { let context = (context).clone(); Arc::new(move |e| {
        if i32::from_ne_bytes((e).to_ne_bytes()) <= i32::from_ne_bytes((raw).to_ne_bytes()) {
            return false;
        }
        let current = context.progressive_break_opportunities.get(&(progressive));
        let resulting = context.progressive_break_opportunities.get(&(e));
        return match &(current) { Some(__option16) => resulting.is_some() && __option16.span_range.start == ((resulting).as_ref().unwrap().span_range).clone().start && __option16.span_range.end == ((resulting).as_ref().unwrap().span_range).clone().end &&
(i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((resulting).as_ref().unwrap().tier)).to_ne_bytes())) < (i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(__option16.tier)).to_ne_bytes())), None
=> false };
}) };
        let mut filtered: Vec<u32> = vec![];
        let mut i = u32::wrapping_sub(raw, self.candidate_window);
        if i32::from_ne_bytes((i).to_ne_bytes()) < (i32::from_ne_bytes((u32::wrapping_add(start, 1)).to_ne_bytes())) {
            i = u32::wrapping_add(start, 1);
        }
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((raw).to_ne_bytes()) {
            if (!ends_with_mandatory || i != u32::wrapping_sub(segment_end_exclusive, 1)) && !context.unbreakable_ranges.contains_boundary(i) && (is_promotion(i) || ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(start, raw, i,
(context.progressive_break_opportunities).clone(), Some((context.adjusted_clusters).clone()), Some(limit), Some((context.cjk_inter_char_boundaries).clone()), Some(context.max_cjk_stretch_per_gap), Some((context.sino_western_boundaries).clone()),
Some(context.sino_western_stretch_cap))) && (i == segment_end_exclusive || ParagraphDpLineBreaker::paragraph_dp_line_breaker_range_has_only_non_control_clusters(start, i, (context.non_rendering_control_clusters).clone())) {
                filtered.push(i);
            }
            i = u32::wrapping_add(i, 1);
        }
        for &e in &compressed {
            if (!ends_with_mandatory || e != u32::wrapping_sub(segment_end_exclusive, 1)) && !context.unbreakable_ranges.contains_boundary(e) && (is_promotion(e) || ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(start, raw, e,
(context.progressive_break_opportunities).clone(), Some((context.adjusted_clusters).clone()), Some(limit), Some((context.cjk_inter_char_boundaries).clone()), Some(context.max_cjk_stretch_per_gap), Some((context.sino_western_boundaries).clone()),
Some(context.sino_western_stretch_cap))) && (e == segment_end_exclusive || ParagraphDpLineBreaker::paragraph_dp_line_breaker_range_has_only_non_control_clusters(start, e, (context.non_rendering_control_clusters).clone())) {
                filtered.push(e);
            }
        }
        let mut clean: Vec<u32> = vec![];
        for &e in &filtered {
            if e == segment_end_exclusive || (match &(context.forbidden_line_start_clusters) { None => true, Some(__option17) => !__option17.has(&(e)) }) && !context.forbidden_line_end_clusters.has(&(u32::wrapping_sub(e, 1))) {
                clean.push(e);
            }
        }
        let mut pool = if u32::try_from((clean.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { filtered } else { clean };
        let mut promotions: Vec<u32> = vec![];
        for &e in &pool {
            if is_promotion(e) {
                promotions.push(e);
            }
        }
        if i32::from_ne_bytes((u32::try_from((promotions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            let mut best = 999999u32;
            let promoted = (context.progressive_break_opportunities.get(&(promotions[0usize])).as_ref().unwrap().span_range).clone();
            for &e in &promotions {
                let o = context.progressive_break_opportunities.get(&(e));
                if i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(o.as_ref().unwrap().tier)).to_ne_bytes()) < (i32::from_ne_bytes((best).to_ne_bytes())) {
                    best = ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(o.as_ref().unwrap().tier);
                }
            }
            let mut preferred: Vec<u32> = vec![];
            for &e in &pool {
                let o = context.progressive_break_opportunities.get(&(e));
                if match &(o) { None => true, Some(__option18) => __option18.span_range.start != promoted.start } || ((o).as_ref().unwrap().span_range).clone().end != promoted.end ||
(i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((o).as_ref().unwrap().tier)).to_ne_bytes())) <= i32::from_ne_bytes((best).to_ne_bytes()) {
                    preferred.push(e);
                }
            }
            pool = preferred;
        }
        if i32::from_ne_bytes((baseline).to_ne_bytes()) >= i32::from_ne_bytes((u32::wrapping_add(start, 1)).to_ne_bytes()) && (i32::from_ne_bytes((baseline).to_ne_bytes())) <= i32::from_ne_bytes((segment_end_exclusive).to_ne_bytes()) && u32::try_from((promotions.len()) &
0xFFFF_FFFF).unwrap_or(0) == 0 {
            pool.push(baseline);
        }
        let mut unique: Vec<u32> = vec![];
        for &e in &pool {
            let mut seen = false;
            {
                let mut _g = 0u32;
                while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((unique.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let v = unique[usize::try_from(_g).unwrap_or(0)];
                    _g = u32::wrapping_add(_g, 1);
                    if v == e {
                        seen = true;
                    }
                }
            }
            if !seen {
                unique.push(e);
            }
        }
        return if i32::from_ne_bytes((u32::try_from((unique.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) { unique } else { vec![
    if i32::from_ne_bytes((baseline).to_ne_bytes()) < (i32::from_ne_bytes((u32::wrapping_add(start, 1)).to_ne_bytes())) { u32::wrapping_add(start, 1) } else { baseline },
] };
    }

    fn edge_geometry(&self, context: DpContext, line: LineCandidate, is_segment_last: bool, hyphen_end: bool) -> Result<EdgeGeometry, TextRangeError> {
        let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(context.max_width, context.first_line_indent, line.cluster_range.start);
        let in_measure = line.cluster_range.clone();
        let overflow = line.adjusted_width - limit;
        let orphan = if !is_segment_last && in_measure.start == in_measure.end { i32::from_ne_bytes((self.leave_ragged_penalty).to_ne_bytes()) as f64 } else { 0.0f64 } as f64;
        let hyphen = if hyphen_end { self.synthetic_hyphen_break_penalty } else { 0.0f64 };
        let r#ref = if context.d_ref < (1.0f64) { 1.0f64 } else { context.d_ref };
        if overflow > (0.0f64) {
            let gaps = if i32::from_ne_bytes((context.gap_count((in_measure).clone())).to_ne_bytes()) < (1) { 1 } else { context.gap_count((in_measure).clone()) };
            let d = (overflow / format!("{}", { let v: u32 = gaps; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0)) * self.compression_visibility;
            return Ok(EdgeGeometry::new(orphan + hyphen + (d * d / r#ref) * self.raggedness_weight, false)?);
        }
        let deficit = if is_segment_last { 0.0f64 } else { { let __min_a = (limit - line.adjusted_width) as f64; let __min_b = 0.0f64 as f64; if __min_a.is_nan() || __min_b.is_nan() { f64::NAN } else { if __min_a > __min_b { __min_a } else if __min_b > __min_a { __min_b } else if
__min_a == 0.0 && __min_b == 0.0 { if __min_a.is_sign_negative() { __min_b } else { __min_a } } else { __min_a } } } };
        let sino_gaps = context.sino_gap_count((in_measure).clone());
        let cjk_gaps = context.cjk_gap_count((in_measure).clone());
        let sino_fill = if i32::from_ne_bytes((sino_gaps).to_ne_bytes()) > (0) { { let __min_a1 = deficit as f64; let __min_b1 = (format!("{}", (i32::from_ne_bytes((sino_gaps).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * context.sino_western_stretch_cap) as f64; if
__min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 < __min_b1 { __min_a1 } else if __min_b1 < __min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_a1 } else { __min_b1 } } else { __min_a1 } } } } else {
0.0f64 };
        let d_sino = if i32::from_ne_bytes((sino_gaps).to_ne_bytes()) > (0) { sino_fill / format!("{}", (i32::from_ne_bytes((sino_gaps).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) } else { 0.0f64 };
        let cjk_deficit = deficit - sino_fill;
        let d_cjk = if i32::from_ne_bytes((cjk_gaps).to_ne_bytes()) > (0) { cjk_deficit / format!("{}", (i32::from_ne_bytes((cjk_gaps).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) } else { 0.0f64 };
        let residual = if cjk_gaps == 0 { cjk_deficit } else { 0.0f64 };
        return Ok(EdgeGeometry::new(residual * self.raggedness_weight + orphan + hyphen + ((d_sino * d_sino + d_cjk * d_cjk) / r#ref) * self.raggedness_weight, { let __min_a2 = d_sino as f64; let __min_b2 = d_cjk as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else
{ if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 == 0.0 && __min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } } >
(ParagraphDpLineBreaker::PARAGRAPH_DP_LINE_BREAKER_VISIBLE_STRETCH_FLOOR_PX))?);
    }

    fn solve_segment(&self, context: DpContext, segment_start: u32, segment_end_exclusive: u32, ends_with_mandatory: bool) -> Result<Vec<u32>, TextRangeError> {
        let mut states_builder: SortedMapTableBuilder<u32, Vec<EdgeState>> = SortedTable::sorted_table_map_builder::<u32, Vec<EdgeState>>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        states_builder.put(&(segment_start), &(vec![]));
        let mut best_builder: SortedMapTableBuilder<String, EdgeState> = SortedTable::sorted_table_map_builder::<String, EdgeState>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let mut terminal_best: Option<EdgeState> = None;
        let mut start = segment_start;
        while (i32::from_ne_bytes((start).to_ne_bytes())) < (i32::from_ne_bytes((segment_end_exclusive).to_ne_bytes())) {
            let bucket = states_builder.get(&(start));
            let mut incoming: Vec<Option<EdgeState>> = vec![];
            if start == segment_start {
                incoming.push(None);
            } else {
                match &(bucket) {
                    Some(__option19) => {
                        let mut _g = 0u32;
                        while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((__option19.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                            let state = ((__option19)[usize::try_from(_g).unwrap_or(0)]).clone();
                            _g = u32::wrapping_add(_g, 1);
                            incoming.push(Some(state));
                        }
                    }
                    None => {
                    }
                }
            }
            if u32::try_from((incoming.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
                start = u32::wrapping_add(start, 1);
                continue;
            }
            {
                let _g1 = self.candidate_ends((context).clone(), start, segment_end_exclusive, ends_with_mandatory);
                for &e in &_g1 {
                    let last = { let v: u32 = e; i32::from_ne_bytes(v.to_ne_bytes()) } >= i32::from_ne_bytes((segment_end_exclusive).to_ne_bytes());
                    let reason = if !last { LineEndReason::AutoWrap } else { if ends_with_mandatory { LineEndReason::MandatoryBreak } else { LineEndReason::ParagraphEnd } };
                    let line_end = if i32::from_ne_bytes((u32::wrapping_sub(e, 1)).to_ne_bytes()) < (i32::from_ne_bytes((u32::wrapping_sub(segment_end_exclusive, 1)).to_ne_bytes())) { u32::wrapping_sub(e, 1) } else { u32::wrapping_sub(segment_end_exclusive, 1) };
                    let line = context.build_line(IntRange::new(start, line_end), reason)?;
                    let hyphen_end = !last && context.hyphen_break_clusters.has(&(e));
                    let geometry = self.edge_geometry((context).clone(), (line).clone(), last, hyphen_end)?;
                    {
                        let mut _g = 0u32;
                        while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((incoming.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                            let prev = (incoming[usize::try_from(_g).unwrap_or(0)]).clone();
                            _g = u32::wrapping_add(_g, 1);
                            let ph = match &(prev) { None => 0, Some(__option20) => __option20.hyphen_run };
                            let ps = match &(prev) { None => 0, Some(__option21) => __option21.stretch_run };
                            let cost = (match &(prev) { None => 0.0f64, Some(__option22) => __option22.cost }) + geometry.base_cost + (if hyphen_end { self.consecutive_synthetic_hyphen_penalty * format!("{}", { let v: u32 = ph; i32::from_ne_bytes(v.to_ne_bytes())
}).parse::<f64>().unwrap_or(0.0) } else { 0.0f64 }) + (if geometry.visible_stretch { self.consecutive_stretch_penalty * format!("{}", { let v: u32 = ps; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0) } else { 0.0f64 });
                            let mut hr = 0u32;
                            if hyphen_end {
                                hr = if i32::from_ne_bytes((u32::wrapping_add(ph, 1)).to_ne_bytes()) > (i32::from_ne_bytes((ParagraphDpLineBreaker::PARAGRAPH_DP_LINE_BREAKER_HYPHEN_RUN_STATE_CAP).to_ne_bytes())) {
ParagraphDpLineBreaker::PARAGRAPH_DP_LINE_BREAKER_HYPHEN_RUN_STATE_CAP } else { u32::wrapping_add(ph, 1) };
                            }
                            let mut sr = 0u32;
                            if geometry.visible_stretch {
                                sr = if i32::from_ne_bytes((u32::wrapping_add(ps, 1)).to_ne_bytes()) > (i32::from_ne_bytes((ParagraphDpLineBreaker::PARAGRAPH_DP_LINE_BREAKER_STRETCH_RUN_STATE_CAP).to_ne_bytes())) {
ParagraphDpLineBreaker::PARAGRAPH_DP_LINE_BREAKER_STRETCH_RUN_STATE_CAP } else { u32::wrapping_add(ps, 1) };
                            }
                            let key = format!("{}{}{}{}{}{}{}",
            crate::runtime::int_text::IntText::int_text(start),
            ":",
            crate::runtime::int_text::IntText::int_text(e),
            ":",
            crate::runtime::int_text::IntText::int_text(hr),
            ":",
            crate::runtime::int_text::IntText::int_text(sr)
        );
                            let existing = best_builder.get(&(key).to_string());
                            match &(existing) {
                                Some(__option23) => {
                                    if __option23.cost <= cost {
                                    continue;
                                    }
                                }
                                None => {
                                }
                            }
                            let state = EdgeState::new(start, e, hr, sr, cost, (prev).clone())?;
                            best_builder.put(&(key).to_string(), &(state));
                            if last {
                                if match &(terminal_best) { None => true, Some(__option24) => cost < (__option24.cost) } {
                                    terminal_best = Some(state.clone());
                                }
                            } else {
                                let mut next = states_builder.get(&(e));
                                if next.is_none() {
                                    next = Some(vec![].clone());
                                }
                                let mut kept: Vec<EdgeState> = vec![];
                                {
                                    let mut _g = 0u32;
                                    while (_g) < (u32::try_from(((next).as_ref().map_or(0, |v| v.len())) & 0xFFFF_FFFF).unwrap_or(0)) {
                                        let old = ((next).as_ref().unwrap()[usize::try_from(_g).unwrap_or(0)]).clone();
                                        _g = u32::wrapping_add(_g, 1);
                                        if !(old.start == start && old.hyphen_run == hr && old.stretch_run == sr) {
                                            kept.push(old.clone());
                                        }
                                    }
                                }
                                kept.push(state.clone());
                                states_builder.put(&(e), &(kept));
                            }
                        }
                    }
                }
            }
            start = u32::wrapping_add(start, 1);
        }
        if terminal_best.is_none() {
            return Ok(self.greedy_fallback_ends((context).clone(), segment_start, segment_end_exclusive));
        }
        let mut result: Vec<u32> = vec![];
        let mut cursor = (terminal_best).clone();
        while cursor.is_some() {
            result.push((cursor).as_ref().unwrap().end);
            cursor = ((cursor).as_ref().unwrap().parent).as_ref().map(|b| (**b).clone());
        }
        result.reverse();
        return Ok(result);
    }

    fn greedy_fallback_ends(&self, context: DpContext, segment_start: u32, segment_end_exclusive: u32) -> Vec<u32> {
        let mut ends: Vec<u32> = vec![];
        let mut start = segment_start;
        while (i32::from_ne_bytes((start).to_ne_bytes())) < (i32::from_ne_bytes((segment_end_exclusive).to_ne_bytes())) {
            let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(context.max_width, context.first_line_indent, start);
            let raw = LineBreakerLines::line_breaker_lines_find_greedy_end(&context.adjusted_clusters, start, limit, Some(segment_end_exclusive), Some((context.non_rendering_control_clusters).clone()));
            let mut e = if i32::from_ne_bytes((raw).to_ne_bytes()) >= i32::from_ne_bytes((segment_end_exclusive).to_ne_bytes()) { segment_end_exclusive } else {
ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_unbreakables(ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(start, raw, &context.adjusted_clusters, limit, (context.hyphen_break_clusters).clone(),
(context.cjk_inter_char_boundaries).clone(), context.max_cjk_stretch_per_gap, Some((context.sino_western_boundaries).clone()), Some(context.sino_western_stretch_cap)), start, (context.unbreakable_ranges).clone()) };
            if ({ let v: u32 = e; i32::from_ne_bytes(v.to_ne_bytes()) }) <= i32::from_ne_bytes((start).to_ne_bytes()) {
                e = u32::wrapping_add(start, 1);
            }
            ends.push(e);
            start = e;
        }
        return ends;
    }

    fn commit_segment(&self, committed: &mut Vec<LineCandidate>, ends: &Vec<u32>, segment_start: u32, mandatory_end: Option<u32>, context: DpContext, hard_break_after_clusters: SortedSetTable<u32>) -> Result<(), TextRangeError> {
        let mut line_start = segment_start;
        for &chosen_end in ends {
            if i32::from_ne_bytes((line_start).to_ne_bytes()) >= { let v: u32 = chosen_end; i32::from_ne_bytes(v.to_ne_bytes()) } {
                continue;
            }
            let final_line = chosen_end == ends[usize::try_from(u32::wrapping_sub(u32::try_from((ends.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)];
            let reason = if final_line && mandatory_end.is_some() { LineEndReason::MandatoryBreak } else { if final_line { LineEndReason::ParagraphEnd } else { LineEndReason::AutoWrap } };
            let last_index = if final_line && mandatory_end.is_some() { mandatory_end } else { Some(u32::wrapping_sub(chosen_end, 1)) };
            let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(context.max_width, context.first_line_indent, line_start);
            let natural_line = LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, last_index.unwrap_or(0)), &context.natural_clusters, &context.adjusted_clusters, Some(reason), None, None)?;
            let mut compressed: Option<LineCandidate> = None;
            if natural_line.adjusted_width > (limit) && (i32::from_ne_bytes((last_index.unwrap_or(0)).to_ne_bytes())) > (i32::from_ne_bytes((line_start).to_ne_bytes())) {
                let resulting_break = context.progressive_break_opportunities.get(&(chosen_end));
                let raw_greedy = LineBreakerLines::line_breaker_lines_find_greedy_end(&context.adjusted_clusters, line_start, limit, Some(ends[usize::try_from(u32::wrapping_sub(u32::try_from((ends.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]),
Some((context.non_rendering_control_clusters).clone()));
                let original_break = context.progressive_break_opportunities.get(&(ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(line_start, raw_greedy, (context.progressive_break_opportunities).clone(), Some((context.adjusted_clusters).clone()),
Some(limit), Some((context.cjk_inter_char_boundaries).clone()), Some(context.max_cjk_stretch_per_gap), Some((context.sino_western_boundaries).clone()), Some(context.sino_western_stretch_cap))));
                let promotes_progressive_tier = match &(original_break) { Some(__option25) => resulting_break.is_some() && __option25.span_range.start == ((resulting_break).as_ref().unwrap().span_range).clone().start && __option25.span_range.end ==
((resulting_break).as_ref().unwrap().span_range).clone().end && (i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((resulting_break).as_ref().unwrap().tier)).to_ne_bytes())) <
(i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(__option25.tier)).to_ne_bytes())), None => false };
                let result = LineRepair::line_repair_try_push_in(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, line_start), &context.natural_clusters, &context.adjusted_clusters, None, None, None)?,
LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(u32::wrapping_add(line_start, 1), last_index.unwrap_or(0)), &context.natural_clusters, &context.adjusted_clusters, Some(reason), None, None)?, &context.natural_clusters, &context.adjusted_clusters, limit,
&context.shrink_opportunities, self.push_in_penalty, last_index, if promotes_progressive_tier { Some("ProgressiveTechnicalTierPromotion".to_string()) } else { Some("LineAdjustmentPushIn".to_string()) }.clone())?;
                if result.candidate.clone().accepted && result.current.is_none() {
                    compressed = Some((result.previous).clone());
                }
            }
            if final_line {
                committed.push(match &(compressed) { None => natural_line, Some(__option27) => (*__option27).clone() });
                line_start = chosen_end;
                if match &(mandatory_end) { Some(__option29) => line_start == u32::try_from((context.adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None => false } {
                    committed.push(LineBreakerLines::line_breaker_lines_empty_line_candidate(((context.adjusted_clusters[usize::try_from(u32::wrapping_sub(u32::try_from((context.adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end,
Some(LineEndReason::ParagraphEnd))?);
                }
            } else {
                match &(compressed) {
                    Some(__option30) => {
                        committed.push((__option30).clone());
                        line_start = chosen_end;
                    }
                    None => {
                        let committed_end = ProgressiveBreakDecisions::progressive_break_decisions_adjust_break_for_line_end(chosen_end, line_start, (context.forbidden_line_end_clusters).clone());
                        if hard_break_after_clusters.has(&(committed_end)) && (i32::from_ne_bytes((line_start).to_ne_bytes())) < (i32::from_ne_bytes((committed_end).to_ne_bytes())) {
                            committed.push(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(line_start, committed_end), &context.natural_clusters, &context.adjusted_clusters, Some(LineEndReason::MandatoryBreak), None, None)?);
                            line_start = u32::wrapping_add(committed_end, 1);
                        } else {
                            committed.push(LineBreakerLines::line_breaker_lines_close_filled_line(IntRange::new(line_start, u32::wrapping_sub(committed_end, 1)), chosen_end, &context.natural_clusters, &context.adjusted_clusters)?);
                            line_start = committed_end;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn paragraph_dp_line_breaker_range_has_only_non_control_clusters(start: u32, end_exclusive: u32, set: SortedSetTable<u32>) -> bool {
        let mut i = start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((end_exclusive).to_ne_bytes())) {
            if !set.has(&(i)) {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }
}

impl LineBreaker for ParagraphDpLineBreaker {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphDpLineBreaker.ParagraphDpLineBreaker"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn LineBreaker> {
        Box::new(self.clone())
    }

    fn get_strategy_name(&self) -> String {
        return "paragraph-dp".to_string();
    }

    fn break_lines(&self, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: Option<Vec<ShrinkOpportunity>>, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters:
Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: Option<SortedSetTable<u32>>, hyphen_break_clusters: Option<SortedSetTable<u32>>, cjk_inter_char_boundaries:
Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>, line_adjustment_push_in: Option<bool>, _line_adjustment_compress_bias: Option<f64>, hard_break_after_clusters:
Option<SortedSetTable<u32>>, non_rendering_control_clusters: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        if u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
        }
        if u32::try_from((natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "naturalClusters and adjustedClusters must align cluster-for-cluster.".to_string() });
        }
        let shrink = match &(shrink_opportunities) { None => vec![], Some(__option31) => (*__option31).clone() };
        let ranges = match &(unbreakable_ranges) { None => UnbreakableRanges::new(vec![].to_vec()), Some(__option32) => (*__option32).clone() };
        let indent = match &(first_line_indent) { None => 0.0f64, Some(__option33) => *__option33 };
        let forbid_start = (forbidden_line_start_clusters).clone();
        let forbid_end = match &(forbidden_line_end_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
Some(__option34) => (*__option34).clone() };
        let hyphens = match &(hyphen_break_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option35)
=> (*__option35).clone() };
        let cjk = match &(cjk_inter_char_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option36)
=> (*__option36).clone() };
        let sino = match &(sino_western_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option37) =>
(*__option37).clone() };
        let mut gap_boundaries_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((cjk.size()).to_ne_bytes()) {
            gap_boundaries_builder.put(&(cjk.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        for i in 0..u32::from_ne_bytes((sino.size()).to_ne_bytes()) {
            gap_boundaries_builder.put(&(sino.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        let gap_boundaries: SortedSetTable<u32> = gap_boundaries_builder.clone().build();
        let controls = match &(non_rendering_control_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
Some(__option38) => (*__option38).clone() };
        let progressive = match &(progressive_break_opportunities) { None => SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option39) => (*__option39).clone() };
        let hard = match &(hard_break_after_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option40)
=> (*__option40).clone() };
        let hangables = match &(hangable_clusters) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option41) =>
(*__option41).clone() };
        let max_stretch = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option42) => *__option42 };
        let sino_cap = match &(sino_western_stretch_cap) { None => 0.0f64, Some(__option43) => *__option43 };
        let context = DpContext::new(natural_clusters.to_vec(), adjusted_clusters.to_vec(), max_width, (shrink).clone(), (ranges).clone(), indent, (forbidden_line_start_clusters).clone(), (forbid_end).clone(), (hyphens).clone(), (cjk).clone(), max_stretch, (sino).clone(),
sino_cap, (controls).clone(), (gap_boundaries).clone(), max_stretch, line_adjustment_push_in.as_ref().map_or(false, |v| v == &(true)), (progressive).clone())?;
        let mut committed: Vec<LineCandidate> = vec![];
        let mut sorted_breaks: Vec<u32> = vec![];
        if i32::from_ne_bytes((u32::from_ne_bytes((hard.size()).to_ne_bytes())).to_ne_bytes()) > (0) {
            for i in 0..u32::from_ne_bytes((hard.size()).to_ne_bytes()) {
                sorted_breaks.push(hard.at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
            }
        }
        let mut cursor = 0u32;
        let mut segment_start = 0u32;
        while (i32::from_ne_bytes((segment_start).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            while (i32::from_ne_bytes((cursor).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && ({ let v: u32 = sorted_breaks[usize::try_from(cursor).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) <
(i32::from_ne_bytes((segment_start).to_ne_bytes())) {
                cursor = u32::wrapping_add(cursor, 1);
            }
            let mandatory = if i32::from_ne_bytes((cursor).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((sorted_breaks.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(sorted_breaks[usize::try_from(cursor).unwrap_or(0)]) } else { None };
            let end = match &(mandatory) { None => u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), Some(__option44) => u32::wrapping_add(*__option44, 1) };
            let ends = self.solve_segment((context).clone(), segment_start, end, mandatory.is_some())?;
            let _ = self.commit_segment(&mut committed, &ends, segment_start, mandatory, (context).clone(), (hard).clone())?;
            segment_start = end;
        }
        return Ok(LineRepair::line_repair_apply_kinsoku_repairs(&committed, &natural_clusters, &adjusted_clusters, max_width, (self.kinsoku).clone(), &shrink, self.push_in_penalty, self.carry_previous_penalty, self.leave_ragged_penalty, Some((ranges).clone()), Some(indent),
Some((hangables).clone()), Some((match &(extendable_hang_ranges) { None => vec![], Some(__option46) => (*__option46).clone() }).clone()), Some(5), (forbid_start).clone())?);
    }
}

#[derive(Clone, PartialEq)]
pub struct DpContext {
    pub natural_clusters: Vec<Cluster>,
    pub adjusted_clusters: Vec<Cluster>,
    pub max_width: f64,
    pub shrink_opportunities: Vec<ShrinkOpportunity>,
    pub unbreakable_ranges: UnbreakableRanges,
    pub first_line_indent: f64,
    pub forbidden_line_start_clusters: Option<SortedSetTable<u32>>,
    pub forbidden_line_end_clusters: SortedSetTable<u32>,
    pub hyphen_break_clusters: SortedSetTable<u32>,
    pub cjk_inter_char_boundaries: SortedSetTable<u32>,
    pub max_cjk_stretch_per_gap: f64,
    pub sino_western_boundaries: SortedSetTable<u32>,
    pub sino_western_stretch_cap: f64,
    pub non_rendering_control_clusters: SortedSetTable<u32>,
    pub gap_boundaries: SortedSetTable<u32>,
    pub d_ref: f64,
    pub allow_compression_edges: bool,
    pub progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>,
    pub(crate) gap_prefix: Vec<u32>,
    pub(crate) sino_prefix: Vec<u32>,
    pub(crate) cjk_prefix: Vec<u32>,
    pub(crate) natural_prefix: Vec<f64>,
    pub(crate) adjusted_prefix: Vec<f64>,
    pub(crate) shrink_prefix: Vec<f64>,
    pub(crate) line_end_only_capacity: Vec<f64>,
}

impl DpContext {
    pub fn new(natural_clusters: Vec<Cluster>, adjusted_clusters: Vec<Cluster>, max_width: f64, shrink_opportunities: Vec<ShrinkOpportunity>, unbreakable_ranges: UnbreakableRanges, first_line_indent: f64, forbidden_line_start_clusters: Option<SortedSetTable<u32>>,
forbidden_line_end_clusters: SortedSetTable<u32>, hyphen_break_clusters: SortedSetTable<u32>, cjk_inter_char_boundaries: SortedSetTable<u32>, max_cjk_stretch_per_gap: f64, sino_western_boundaries: SortedSetTable<u32>, sino_western_stretch_cap: f64, non_rendering_control_clusters:
SortedSetTable<u32>, gap_boundaries: SortedSetTable<u32>, d_ref: f64, allow_compression_edges: bool, progressive_break_opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>) -> Result<Self, TextRangeError> {
    let mut gap_prefix = vec![];
    let mut sino_prefix = vec![];
    let mut cjk_prefix = vec![];
    let mut adjusted_prefix = vec![];
    let mut natural_prefix = vec![];
    let mut shrink_prefix = vec![];
    let mut line_end_only_capacity = vec![];
        let n = u32::try_from((adjusted_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let mut init = 0u32;
        while (i32::from_ne_bytes((init).to_ne_bytes())) <= i32::from_ne_bytes((n).to_ne_bytes()) {
            gap_prefix.push(0);
            sino_prefix.push(0);
            cjk_prefix.push(0);
            adjusted_prefix.push(0.0f64);
            init = u32::wrapping_add(init, 1);
        }
        init = 0u32;
        while (i32::from_ne_bytes((init).to_ne_bytes())) <= i32::from_ne_bytes((u32::try_from((natural_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) {
            natural_prefix.push(0.0f64);
            init = u32::wrapping_add(init, 1);
        }
        let mut k = 0u32;
        while (i32::from_ne_bytes((k).to_ne_bytes())) < (i32::from_ne_bytes((n).to_ne_bytes())) {
            { while gap_prefix.len() <= usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0) { gap_prefix.push(0); } gap_prefix[usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0)] = u32::wrapping_add(gap_prefix[usize::try_from(k).unwrap_or(0)], if gap_boundaries.has(&(k)) { 1
} else { 0 }); };
            { while sino_prefix.len() <= usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0) { sino_prefix.push(0); } sino_prefix[usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0)] = u32::wrapping_add(sino_prefix[usize::try_from(k).unwrap_or(0)], if
sino_western_boundaries.has(&(k)) { 1 } else { 0 }); };
            { while cjk_prefix.len() <= usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0) { cjk_prefix.push(0); } cjk_prefix[usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0)] = u32::wrapping_add(cjk_prefix[usize::try_from(k).unwrap_or(0)], if
cjk_inter_char_boundaries.has(&(k)) { 1 } else { 0 }); };
            { while natural_prefix.len() <= usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0) { natural_prefix.push(0.0); } natural_prefix[usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0)] = natural_prefix[usize::try_from(k).unwrap_or(0)] +
natural_clusters[usize::try_from(k).unwrap_or(0)].advance; };
            { while adjusted_prefix.len() <= usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0) { adjusted_prefix.push(0.0); } adjusted_prefix[usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0)] = adjusted_prefix[usize::try_from(k).unwrap_or(0)] +
adjusted_clusters[usize::try_from(k).unwrap_or(0)].advance; };
            k = u32::wrapping_add(k, 1);
        }
        init = 0u32;
        while (i32::from_ne_bytes((init).to_ne_bytes())) <= i32::from_ne_bytes((n).to_ne_bytes()) {
            shrink_prefix.push(0.0f64);
            init = u32::wrapping_add(init, 1);
        }
        for _ in 0..n {
            line_end_only_capacity.push(0.0f64);
        }
        for opp in &shrink_opportunities {
            if opp.capacity <= 0 as f64 || (opp.cluster_index) > 2147483647 || (i32::from_ne_bytes((opp.cluster_index).to_ne_bytes())) >= i32::from_ne_bytes((n).to_ne_bytes()) {
                continue;
            }
            if opp.line_end_only {
                line_end_only_capacity[usize::try_from(opp.cluster_index).unwrap_or(0)] += opp.capacity;
            } else {
                shrink_prefix[usize::try_from(u32::wrapping_add(opp.cluster_index, 1)).unwrap_or(0)] += opp.capacity;
            }
        }
        k = 0u32;
        while (i32::from_ne_bytes((k).to_ne_bytes())) < (i32::from_ne_bytes((n).to_ne_bytes())) {
            shrink_prefix[usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0)] += shrink_prefix[usize::try_from(k).unwrap_or(0)];
            k = u32::wrapping_add(k, 1);
        }
        Ok(Self {
            natural_clusters,
            adjusted_clusters,
            max_width,
            shrink_opportunities,
            unbreakable_ranges,
            first_line_indent,
            forbidden_line_start_clusters,
            forbidden_line_end_clusters,
            hyphen_break_clusters,
            cjk_inter_char_boundaries,
            max_cjk_stretch_per_gap,
            sino_western_boundaries,
            sino_western_stretch_cap,
            non_rendering_control_clusters,
            gap_boundaries,
            d_ref,
            allow_compression_edges,
            progressive_break_opportunities,
            gap_prefix: gap_prefix,
            sino_prefix: sino_prefix,
            cjk_prefix: cjk_prefix,
            natural_prefix: natural_prefix,
            adjusted_prefix: adjusted_prefix,
            shrink_prefix: shrink_prefix,
            line_end_only_capacity: line_end_only_capacity,
        })
    }

    pub fn build_line(&self, cluster_range: IntRange, end_reason: LineEndReason) -> Result<LineCandidate, TextRangeError> {
        return Ok(LineCandidate::new((cluster_range).clone(), TextRange::new(((self.adjusted_clusters[usize::try_from(cluster_range.start).unwrap_or(0)]).clone().range).clone().start,
((self.adjusted_clusters[usize::try_from(cluster_range.end).unwrap_or(0)]).clone().range).clone().end)?, self.natural_prefix[usize::try_from(u32::wrapping_add(cluster_range.end, 1)).unwrap_or(0)] - self.natural_prefix[usize::try_from(cluster_range.start).unwrap_or(0)],
self.adjusted_prefix[usize::try_from(u32::wrapping_add(cluster_range.end, 1)).unwrap_or(0)] - self.adjusted_prefix[usize::try_from(cluster_range.start).unwrap_or(0)], Some(end_reason), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging()))?);
    }

    pub fn gap_count(&self, range: IntRange) -> u32 {
        return if range.get_is_empty() { 0 } else { u32::wrapping_sub(self.gap_prefix[usize::try_from(range.end).unwrap_or(0)], self.gap_prefix[usize::try_from(range.start).unwrap_or(0)]) };
    }

    pub fn sino_gap_count(&self, range: IntRange) -> u32 {
        return if range.get_is_empty() { 0 } else { u32::wrapping_sub(self.sino_prefix[usize::try_from(range.end).unwrap_or(0)], self.sino_prefix[usize::try_from(range.start).unwrap_or(0)]) };
    }

    pub fn cjk_gap_count(&self, range: IntRange) -> u32 {
        return if range.get_is_empty() { 0 } else { u32::wrapping_sub(self.cjk_prefix[usize::try_from(range.end).unwrap_or(0)], self.cjk_prefix[usize::try_from(range.start).unwrap_or(0)]) };
    }

    pub fn shrink_capacity(&self, range: IntRange) -> f64 {
        return (self.shrink_prefix[usize::try_from(u32::wrapping_add(range.end, 1)).unwrap_or(0)] - self.shrink_prefix[usize::try_from(range.start).unwrap_or(0)]) + self.line_end_only_capacity[usize::try_from(range.end).unwrap_or(0)];
    }
}

#[derive(Clone, PartialEq)]
pub struct EdgeState {
    pub start: u32,
    pub end: u32,
    pub hyphen_run: u32,
    pub stretch_run: u32,
    pub cost: f64,
    pub parent: Option<Box<EdgeState>>,
}

impl EdgeState {
    pub fn new(start: u32, end: u32, hyphen_run: u32, stretch_run: u32, cost: f64, parent: Option<EdgeState>) -> Result<Self, TextRangeError> {
        Ok(Self {
            start,
            end,
            hyphen_run,
            stretch_run,
            cost,
            parent: match parent { Some(v) => Some(Box::new(v)), None => None },
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct EdgeGeometry {
    pub base_cost: f64,
    pub visible_stretch: bool,
}

impl EdgeGeometry {
    pub fn new(base_cost: f64, visible_stretch: bool) -> Result<Self, TextRangeError> {
        Ok(Self {
            base_cost,
            visible_stretch,
        })
    }
}
