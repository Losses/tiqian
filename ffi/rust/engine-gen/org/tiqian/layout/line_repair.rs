use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::KinsokuRule;
use crate::org::tiqian::layout::line_breaker::LineBreakerLines;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::line_optimization::RepairCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Clone, PartialEq)]
pub struct PushInResult {
    pub previous: LineCandidate,
    pub current: Option<LineCandidate>,
    pub candidate: RepairCandidate,
}

impl PushInResult {
    pub fn new(previous: LineCandidate, current: Option<LineCandidate>, candidate: RepairCandidate) -> Self {
        Self {
            previous,
            current,
            candidate,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PushInResult(")); __s += &(UString::from("previous=")); __s += UString::from(format!("{}", (self.previous).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("current=")); __s += (match &(self.current) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("candidate=")); __s += UString::from(format!("{}", (self.candidate).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, Copy)]
pub struct LineRepair;

impl LineRepair {
    pub fn line_repair_apply_kinsoku_repairs(initial: &Vec<LineCandidate>, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, kinsoku: Box<dyn KinsokuRule>, shrink_opportunities: &Vec<ShrinkOpportunity>, push_in_penalty: u32, carry_previous_penalty: u32, leave_ragged_penalty: u32, unbreakable_ranges: Option<UnbreakableRanges>, first_line_indent: Option<f64>, hangable_clusters: Option<SortedSetTable<u32>>, extendable_hang_ranges: Option<Vec<IntRange>>, hang_penalty: Option<u32>, forbidden_line_start_clusters: Option<SortedSetTable<u32>>) -> Result<LineSolution, TextRangeError> {
        if i32::from_ne_bytes(((u32::try_from((initial.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) < (2) {
            return Ok(LineSolution::new(Some((initial).clone()), Some(0 as f64))?);
        }
        let ranges = match &(unbreakable_ranges) { None => (*crate::org::tiqian::layout::progressive_break_decisions::UNBREAKABLE_RANGES_EMPTY).clone(), Some(__option1) => (*__option1).clone() };
        let indent = match &(first_line_indent) { None => 0 as f64, Some(__option2) => *__option2 };
        let hangables = match &(hangable_clusters) { None => LineRepair::line_repair_empty_int_set(), Some(__option3) => (*__option3).clone() };
        let extendables = match &(extendable_hang_ranges) { None => vec![], Some(__option4) => (*__option4).clone() };
        let hang_cost = match &(hang_penalty) { None => 5, Some(__option5) => *__option5 };
        let mut mutable = initial.clone();
        let mut i = 1u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((mutable.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let curr = (mutable[usize::try_from(i).unwrap_or(0)]).clone();
            let first_index = curr.cluster_range.start;
            let prev = (mutable[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone();
            if prev.end_reason == LineEndReason::MandatoryBreak || curr.cluster_range.get_is_empty() {
                i = u32::wrapping_add(i, 1);
                continue;
            }
            let first_cluster = (adjusted_clusters[usize::try_from(first_index).unwrap_or(0)]).clone();
            let forbidden = match &(forbidden_line_start_clusters) { Some(__option6) => __option6.has(&(first_index)), None => kinsoku.forbidden_at_line_start((first_cluster).clone()) };
            if !forbidden {
                i = u32::wrapping_add(i, 1);
                continue;
            }
            let mut repair_candidates: Vec<RepairCandidate> = vec![];
            let push_in = LineRepair::line_repair_try_push_in((prev).clone(), (curr).clone(), &natural_clusters, &adjusted_clusters, ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, indent, prev.cluster_range.start), &shrink_opportunities, push_in_penalty, None, Some(UString::from("ForbiddenAtLineStart")))?;
            repair_candidates.push((push_in.candidate).clone());
            if push_in.candidate.clone().accepted {
                mutable[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] = (push_in.previous).clone();
                match &(push_in.current) {
                    None => {
                        { let _a = &mut (mutable); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes(((i) as i32).to_ne_bytes()); let splice_index = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos }; let _len = 1i32; let splice_count = if _len < 0 { 0 } else { let _rest = _n - splice_index; if _len < _rest { _len } else { _rest } }; let splice_removed: Vec<_> = _a.drain(usize::try_from(splice_index).unwrap_or(0)..usize::try_from(splice_index + splice_count).unwrap_or(0)).collect(); splice_removed };
                    }
                    Some(__option7) => {
                        mutable[usize::try_from(i).unwrap_or(0)] = (*__option7).clone();
                    }
                }
                continue;
            }
            let offender_index = curr.cluster_range.start;
            let existing_hanging: SortedSetTable<u32> = prev.hanging_cluster_indices.clone();
            let mut extends_contextual_hang = false;
            if i32::from_ne_bytes(((u32::from_ne_bytes(((existing_hanging.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes()) > (0) && u32::wrapping_add(prev.cluster_range.end, 1) == offender_index {
                let mut gi = 0u32;
                while (i32::from_ne_bytes(((gi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((extendables.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let group = (extendables[usize::try_from(gi).unwrap_or(0)]).clone();
                    if i32::from_ne_bytes(((offender_index) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((group.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((offender_index) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((group.end) as i32).to_ne_bytes()) {
                        let mut all = true;
                        let mut hi = 0u32;
                        while (i32::from_ne_bytes(((hi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((existing_hanging.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
                            let idx = existing_hanging.at(i32::from_ne_bytes(((hi) as i32).to_ne_bytes()));
                            if i32::from_ne_bytes(((idx) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((group.start) as i32).to_ne_bytes())) || (i32::from_ne_bytes(((idx) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((group.end) as i32).to_ne_bytes())) {
                                all = false;
                                break;
                            }
                            hi = u32::wrapping_add(hi, 1);
                        }
                        if all {
                            extends_contextual_hang = true;
                            break;
                        }
                    }
                    gi = u32::wrapping_add(gi, 1);
                }
            }
            if hangables.has(&(offender_index)) && (u32::from_ne_bytes(((existing_hanging.size()) as u32).to_ne_bytes()) == 0 || extends_contextual_hang) {
                let merge_end_index = LineRepair::line_repair_mandatory_break_tail_end((curr).clone(), offender_index, &adjusted_clusters);
                let hang_candidate = RepairCandidate::new(&(UStr::new(&[72,97,110,103])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), offender_index, hang_cost, true, None, None, None, Some(0 as f64), Some(0 as f64), Some(0 as f64))?;
                repair_candidates.push(hang_candidate.clone());
                let merged_range = IntRange::new(prev.cluster_range.start, merge_end_index);
                let mut merged_terms: Vec<f64> = vec![];
                let mut wi = u32::wrapping_add(prev.cluster_range.end, 1);
                while (i32::from_ne_bytes(((wi) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((merge_end_index) as i32).to_ne_bytes()) {
                    merged_terms.push(natural_clusters[usize::try_from(wi).unwrap_or(0)].advance);
                    wi = u32::wrapping_add(wi, 1);
                }
                let merged_natural = prev.natural_width + AccurateSum::accurate_sum_of(&merged_terms);
                let mut hang_indices: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
                let mut ai = 0u32;
                while (i32::from_ne_bytes(((ai) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((existing_hanging.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
                    hang_indices.put(&(existing_hanging.at(i32::from_ne_bytes(((ai) as i32).to_ne_bytes()))));
                    ai = u32::wrapping_add(ai, 1);
                }
                let mut xi = offender_index;
                while (i32::from_ne_bytes(((xi) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((merge_end_index) as i32).to_ne_bytes()) {
                    hang_indices.put(&(xi));
                    xi = u32::wrapping_add(xi, 1);
                }
                let mut candidate_list = prev.repair_candidates.clone();
                let mut cpi = 0u32;
                while (i32::from_ne_bytes(((cpi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    candidate_list.push((repair_candidates[usize::try_from(cpi).unwrap_or(0)]).clone());
                    cpi = u32::wrapping_add(cpi, 1);
                }
                mutable[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] = LineCandidate::new((merged_range).clone(), TextRange::new(((adjusted_clusters[usize::try_from(merged_range.start).unwrap_or(0)]).clone().range).clone().start, ((adjusted_clusters[usize::try_from(merge_end_index).unwrap_or(0)]).clone().range).clone().end)?, merged_natural, prev.adjusted_width, if merge_end_index == curr.cluster_range.end { Some(curr.end_reason) } else { Some(prev.end_reason) }, Some(RepairOption::Hang { penalty: hang_cost, reason: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ForbiddenAtLineStart:")); __s += (first_cluster.text).to_ustring().as_ustr(); __s += &(UString::from(":hang")); __s }).as_str()), offender_cluster_index: offender_index }), Some((candidate_list).clone()), Some(hang_indices.clone().build()))?;
                if merge_end_index == curr.cluster_range.end {
                    { let _a = &mut (mutable); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes(((i) as i32).to_ne_bytes()); let splice_index1 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos }; let _len = 1i32; let splice_count1 = if _len < 0 { 0 } else { let _rest = _n - splice_index1; if _len < _rest { _len } else { _rest } }; let splice_removed1: Vec<_> = _a.drain(usize::try_from(splice_index1).unwrap_or(0)..usize::try_from(splice_index1 + splice_count1).unwrap_or(0)).collect(); splice_removed1 };
                } else {
                    mutable[usize::try_from(i).unwrap_or(0)] = LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(u32::wrapping_add(merge_end_index, 1), curr.cluster_range.end), &natural_clusters, &adjusted_clusters, Some(curr.end_reason), None, None)?;
                }
                continue;
            }
            let can_carry = i32::from_ne_bytes(((prev.cluster_range.start) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((prev.cluster_range.end) as i32).to_ne_bytes()));
            if !can_carry {
                repair_candidates.push(RepairCandidate::new(&(UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), first_index, carry_previous_penalty, false, Some(UString::from("no-room-to-carry")), None, None, Some(0 as f64), Some(0 as f64), Some(0 as f64))?);
                repair_candidates.push(RepairCandidate::new(&(UStr::new(&[76,101,97,118,101,82,97,103,103,101,100])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), first_index, leave_ragged_penalty, true, None, None, None, Some(0 as f64), Some(0 as f64), Some(0 as f64))?);
                mutable[usize::try_from(i).unwrap_or(0)] = LineCandidate::new((curr.cluster_range).clone(), (curr.source_range).clone(), curr.natural_width, curr.adjusted_width, Some(curr.end_reason), Some(RepairOption::LeaveRagged { penalty: leave_ragged_penalty, reason: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ForbiddenAtLineStart:")); __s += (first_cluster.text).to_ustring().as_ustr(); __s += &(UString::from(":no-room-to-carry")); __s }).as_str()), offender_cluster_index: first_index }), Some((repair_candidates).clone()), Some((curr.hanging_cluster_indices).clone()))?;
                i = u32::wrapping_add(i, 1);
                continue;
            }
            let carried_index = prev.cluster_range.end;
            if ranges.contains_boundary(carried_index) {
                repair_candidates.push(RepairCandidate::new(&(UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), first_index, carry_previous_penalty, false, Some(UString::from("carry-would-split-mourning-span")), None, Some(carried_index), Some(0 as f64), Some(0 as f64), Some(0 as f64))?);
                repair_candidates.push(RepairCandidate::new(&(UStr::new(&[76,101,97,118,101,82,97,103,103,101,100])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), first_index, leave_ragged_penalty, true, None, None, None, Some(0 as f64), Some(0 as f64), Some(0 as f64))?);
                mutable[usize::try_from(i).unwrap_or(0)] = LineCandidate::new((curr.cluster_range).clone(), (curr.source_range).clone(), curr.natural_width, curr.adjusted_width, Some(curr.end_reason), Some(RepairOption::LeaveRagged { penalty: leave_ragged_penalty, reason: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ForbiddenAtLineStart:")); __s += (first_cluster.text).to_ustring().as_ustr(); __s += &(UString::from(":carry-would-split-mourning-span")); __s }).as_str()), offender_cluster_index: first_index }), Some((repair_candidates).clone()), Some((curr.hanging_cluster_indices).clone()))?;
                i = u32::wrapping_add(i, 1);
                continue;
            }
            let new_prev_range = IntRange::new(prev.cluster_range.start, u32::wrapping_sub(carried_index, 1));
            let new_curr_range = IntRange::new(carried_index, curr.cluster_range.end);
            let carried_current = LineBreakerLines::line_breaker_lines_rebuild_line((new_curr_range).clone(), &natural_clusters, &adjusted_clusters, Some(curr.end_reason), None, None)?;
            if carried_current.adjusted_width > (max_width) {
                repair_candidates.push(RepairCandidate::new(&(UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), first_index, carry_previous_penalty, false, Some(UString::from("carry-overflows")), None, Some(carried_index), Some(0 as f64), Some(0 as f64), Some(0 as f64))?);
                repair_candidates.push(RepairCandidate::new(&(UStr::new(&[76,101,97,118,101,82,97,103,103,101,100])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), first_index, leave_ragged_penalty, true, None, None, None, Some(0 as f64), Some(0 as f64), Some(0 as f64))?);
                mutable[usize::try_from(i).unwrap_or(0)] = LineCandidate::new((curr.cluster_range).clone(), (curr.source_range).clone(), curr.natural_width, curr.adjusted_width, Some(curr.end_reason), Some(RepairOption::LeaveRagged { penalty: leave_ragged_penalty, reason: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ForbiddenAtLineStart:")); __s += (first_cluster.text).to_ustring().as_ustr(); __s += &(UString::from(":carry-overflows")); __s }).as_str()), offender_cluster_index: first_index }), Some((repair_candidates).clone()), Some((curr.hanging_cluster_indices).clone()))?;
                i = u32::wrapping_add(i, 1);
                continue;
            }
            repair_candidates.push(RepairCandidate::new(&(UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), first_index, carry_previous_penalty, true, None, None, Some(carried_index), Some(0 as f64), Some(0 as f64), Some(0 as f64))?);
            mutable[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] = LineBreakerLines::line_breaker_lines_rebuild_line((new_prev_range).clone(), &natural_clusters, &adjusted_clusters, Some(prev.end_reason), None, None)?;
            mutable[usize::try_from(i).unwrap_or(0)] = LineCandidate::new((carried_current.cluster_range).clone(), (carried_current.source_range).clone(), carried_current.natural_width, carried_current.adjusted_width, Some(carried_current.end_reason), Some(RepairOption::CarryPrevious { penalty: carry_previous_penalty, reason: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ForbiddenAtLineStart:")); __s += (first_cluster.text).to_ustring().as_ustr(); __s += &(UString::from(":carried=")); __s += ((adjusted_clusters[usize::try_from(carried_index).unwrap_or(0)]).clone().text).to_ustring().as_ustr(); __s }).as_str()), offender_cluster_index: first_index, carried_cluster_index: carried_index }), Some((repair_candidates).clone()), Some((carried_current.hanging_cluster_indices).clone()))?;
            i = u32::wrapping_add(i, 1);
        }
        let mut badness_terms: Vec<f64> = vec![];
        for bi in 0..match u32::try_from(mutable.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            badness_terms.push(match &((mutable[usize::try_from(bi).unwrap_or(0)]).clone().repair) { None => 0.0f64, Some(__option9) => format!("{}", (i32::from_ne_bytes(((RepairOptions::repair_options_penalty((*__option9).clone())) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * 1.0f64 });
        }
        let total_badness = AccurateSum::accurate_sum_of(&badness_terms);
        return Ok(LineSolution::new(Some((mutable).clone()), Some(total_badness))?);
    }

    pub fn line_repair_try_push_in(prev: LineCandidate, curr: LineCandidate, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: &Vec<ShrinkOpportunity>, push_in_penalty: u32, merge_through_cluster_index: Option<u32>, reason_code: Option<UString>) -> Result<PushInResult, TextRangeError> {
        let code = match &(reason_code) { None => UString::from("ForbiddenAtLineStart"), Some(__option10) => __option10.to_ustring() };
        let offender_index = match &(merge_through_cluster_index) { None => curr.cluster_range.start, Some(__option11) => *__option11 };
        if i32::from_ne_bytes(((offender_index) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((curr.cluster_range.start) as i32).to_ne_bytes())) || (i32::from_ne_bytes(((offender_index) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((curr.cluster_range.end) as i32).to_ne_bytes())) {
            return Err(TextRangeError::Message { text: UString::from("PushIn merge-through cluster must belong to the current line.") });
        }
        let merge_end_index = LineRepair::line_repair_mandatory_break_tail_end((curr).clone(), offender_index, &adjusted_clusters);
        let expanded_range = IntRange::new(prev.cluster_range.start, merge_end_index);
        let expanded = LineBreakerLines::line_breaker_lines_rebuild_line((expanded_range).clone(), &natural_clusters, &adjusted_clusters, None, None, None)?;
        let overflow = expanded.adjusted_width - max_width;
        let mut pipeline_result: Vec<ShrinkOpportunity> = Vec::new();
        for v in shrink_opportunities {
            if i32::from_ne_bytes(((v.cluster_index) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((expanded_range.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((v.cluster_index) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((expanded_range.end) as i32).to_ne_bytes()) && (v.capacity) > (0 as f64) && (!v.line_end_only || v.cluster_index == offender_index) {
                pipeline_result.push(v.clone());
            }
        }
        let _this = (pipeline_result).clone();
        let capacity = _this.len();
        let mut pipeline_result1 = Vec::with_capacity(capacity);
        for pipeline_index1 in 0..match u32::try_from(_this.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (_this[usize::try_from(pipeline_index1).unwrap_or(0)]).clone();
            pipeline_result1.push(if v.cluster_index == offender_index && (v.channel == ShrinkChannel::TrailingGlue || v.channel == ShrinkChannel::LeadingAndTrailingGlue) { ShrinkOpportunity::new(v.cluster_index, 1u32, v.capacity, v.channel, Some(v.line_end_only)) } else { v });
        }
        let in_line = (pipeline_result1).clone();
        let mut capacity_terms: Vec<f64> = vec![];
        for ci in 0..match u32::try_from(in_line.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            capacity_terms.push(in_line[usize::try_from(ci).unwrap_or(0)].capacity);
        }
        let total_capacity = AccurateSum::accurate_sum_of(&capacity_terms);
        if overflow > (total_capacity) {
            let required = if overflow > (0 as f64) { overflow } else { 0 as f64 };
            return Ok(PushInResult::new((prev).clone(), Some((curr).clone()), RepairCandidate::new(&(UStr::new(&[80,117,115,104,73,110])), code.as_ustr(), offender_index, push_in_penalty, false, Some(UString::from("insufficient-capacity")), Some(offender_index), None, Some(0 as f64), Some(required), Some(total_capacity))?));
        }
        let shrink = if overflow > (0 as f64) { overflow } else { 0 as f64 };
        let allocations = if shrink > (0 as f64) { LineRepair::line_repair_distribute_push_in_shrink(&in_line, shrink)? } else { vec![] };
        let offender = (adjusted_clusters[usize::try_from(offender_index).unwrap_or(0)]).clone();
        let candidate = RepairCandidate::new(&(UStr::new(&[80,117,115,104,73,110])), code.as_ustr(), offender_index, push_in_penalty, true, None, Some(offender_index), None, Some(shrink), Some(shrink), Some(total_capacity))?;
        let reason_text = if shrink > (0 as f64) { UString::from(format!("{}", { let mut __s = UString::new(); __s += code.as_ustr(); __s += &(UString::from(":")); __s += (offender.text).to_ustring().as_ustr(); __s += &(UString::from(":pushed-in=")); __s += LineRepair::line_repair_to_portable_debug_string(shrink).as_ustr(); __s += &(UString::from("/")); __s += LineRepair::line_repair_to_portable_debug_string(total_capacity).as_ustr(); __s }).as_str()) } else { UString::from(format!("{}", { let mut __s = UString::new(); __s += code.as_ustr(); __s += &(UString::from(":")); __s += (offender.text).to_ustring().as_ustr(); __s += &(UString::from(":fits-no-shrink")); __s }).as_str()) };
        let mut candidate_list = prev.repair_candidates.clone();
        candidate_list.push(candidate.clone());
        let repaired_previous = LineCandidate::new((expanded.cluster_range).clone(), (expanded.source_range).clone(), expanded.natural_width, expanded.adjusted_width - shrink, if merge_end_index == curr.cluster_range.end { Some(curr.end_reason) } else { Some(prev.end_reason) }, Some(RepairOption::PushIn { penalty: push_in_penalty, reason: reason_text.to_ustring(), offender_cluster_index: offender_index, allocations: allocations.to_vec(), total_shrink: shrink, total_available_capacity: total_capacity }), Some((candidate_list).clone()), Some((expanded.hanging_cluster_indices).clone()))?;
        let repaired_current = if merge_end_index == curr.cluster_range.end { None } else { Some(LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(u32::wrapping_add(merge_end_index, 1), curr.cluster_range.end), &natural_clusters, &adjusted_clusters, Some(curr.end_reason), None, None)?) };
        return Ok(PushInResult::new((repaired_previous).clone(), (repaired_current).clone(), (candidate).clone()));
    }

    pub(crate) fn line_repair_mandatory_break_tail_end(curr: LineCandidate, merge_through_cluster_index: u32, adjusted_clusters: &Vec<Cluster>) -> u32 {
        if curr.end_reason != LineEndReason::MandatoryBreak {
            return merge_through_cluster_index;
        }
        if i32::from_ne_bytes(((merge_through_cluster_index) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((curr.cluster_range.end) as i32).to_ne_bytes()) {
            return merge_through_cluster_index;
        }
        let mut tail_is_zero_width_break = true;
        let mut idx = u32::wrapping_add(merge_through_cluster_index, 1);
        while (i32::from_ne_bytes(((idx) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((curr.cluster_range.end) as i32).to_ne_bytes()) {
            if !(u_string::unit_count(&(((adjusted_clusters[usize::try_from(idx).unwrap_or(0)]).clone().display_text).to_ustring())) == 0 && adjusted_clusters[usize::try_from(idx).unwrap_or(0)].advance == 0 as f64) {
                tail_is_zero_width_break = false;
                break;
            }
            idx = u32::wrapping_add(idx, 1);
        }
        return if tail_is_zero_width_break { curr.cluster_range.end } else { merge_through_cluster_index };
    }

    pub(crate) fn line_repair_distribute_push_in_shrink(opportunities: &Vec<ShrinkOpportunity>, total_shrink: f64) -> Result<Vec<PushInAllocation>, TextRangeError> {
        if u32::try_from((opportunities.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || (total_shrink) <= 0 as f64 {
            return Ok(vec![]);
        }
        let mut allocations: Vec<PushInAllocation> = vec![];
        let mut remaining = total_shrink;
        let mut pipeline_builder: SortedMapTableBuilder<u32, Vec<ShrinkOpportunity>> = SortedTable::sorted_table_map_builder::<u32,
Vec<ShrinkOpportunity>>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for opp in opportunities {
            let pipeline_entry = ShrinkTierEntry { key: opp.tier, value: (*opp).clone() };
            let mut pipeline_bucket = match pipeline_builder.get(&(pipeline_entry.key)) {
                Some(b) => b,
                None => Vec::new(),
            };
            pipeline_bucket.push((pipeline_entry.value).clone());
            pipeline_builder.put(&(pipeline_entry.key), &pipeline_bucket);
        }
        let pipeline_result: SortedMapTable<u32, Vec<ShrinkOpportunity>> = pipeline_builder.clone().build();
        let by_tier: SortedMapTable<u32, Vec<ShrinkOpportunity>> = (pipeline_result).clone();
        let mut tier_idx = 0u32;
        while (i32::from_ne_bytes(((tier_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((by_tier.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            let tier_opps = by_tier.value_at(i32::from_ne_bytes(((tier_idx) as i32).to_ne_bytes()));
            let mut tier_terms: Vec<f64> = vec![];
            for ti in 0..match u32::try_from(tier_opps.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                tier_terms.push(tier_opps[usize::try_from(ti).unwrap_or(0)].capacity);
            }
            let tier_capacity = AccurateSum::accurate_sum_of(&tier_terms);
            if tier_capacity > (0 as f64) {
                let tier_shrink = if remaining < (tier_capacity) { remaining } else { tier_capacity };
                let mut tier_remaining = tier_shrink;
                let ordered = LineRepair::line_repair_sort_by_cluster_index(&tier_opps);
                let mut i2 = 0u32;
                while (i32::from_ne_bytes(((i2) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((ordered.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let opp = (ordered[usize::try_from(i2).unwrap_or(0)]).clone();
                    let is_last = i2 == u32::wrapping_sub(u32::try_from((tier_opps.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
                    let share: f64;
                    if is_last {
                        share = if tier_remaining < (opp.capacity) { tier_remaining } else { opp.capacity };
                    } else {
                        let proportional = tier_shrink * opp.capacity / tier_capacity;
                        share = if proportional < (opp.capacity) { proportional } else { opp.capacity };
                    }
                    if share > (0 as f64) {
                        allocations.push(PushInAllocation::new(opp.cluster_index, share, opp.capacity, Some(opp.channel))?);
                        tier_remaining -= share;
                    }
                    i2 = u32::wrapping_add(i2, 1);
                }
                remaining -= tier_shrink - (if tier_remaining > (0 as f64) { tier_remaining } else { 0 as f64 });
            }
            tier_idx = u32::wrapping_add(tier_idx, 1);
        }
        return Ok(allocations);
    }

    pub(crate) fn line_repair_sort_by_cluster_index(opportunities: &Vec<ShrinkOpportunity>) -> Vec<ShrinkOpportunity> {
        let pipeline_result = {
    let mut _sorted = opportunities.to_vec();
    _sorted.sort_by_key(|opp| opp.cluster_index);
    _sorted
};
        return pipeline_result;
    }

    pub(crate) fn line_repair_to_portable_debug_string(value: f64) -> UString {
        let text = { let mut __s = UString::new(); __s += &(UString::from("")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(value)); __s };
        if u32::from_ne_bytes(((u_string::find_from(&(text), UString::from(".").as_ustr(), 0)) as u32).to_ne_bytes()) == 4294967295u32 && u32::from_ne_bytes(((u_string::find_from(&(text.to_lowercase()), UString::from("e").as_ustr(), 0)) as u32).to_ne_bytes()) == 4294967295u32 {
            return UString::from(format!("{}", { let mut __s = UString::new(); __s += text.as_ustr(); __s += &(UString::from(".0")); __s }).as_str());
        }
        return text;
    }

    pub(crate) fn line_repair_empty_int_set() -> SortedSetTable<u32> {
        let b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        return b.clone().build();
    }

    pub(crate) fn line_repair_is_continuable_zero_shrink_fill_push_in(repair: Option<RepairOption>) -> bool {
        if repair == None {
            return false;
        }
        let r = (repair).as_ref().unwrap().clone();
        return match r {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, .. } => _p4 <= 0.001f64 && (_p1).starts_with(&UString::from("LineAdjustmentPushIn:")),
            RepairOption::Hang { .. } => false,
            RepairOption::CarryPrevious { .. } => false,
            RepairOption::CarryNext { .. } => false,
            RepairOption::LeaveRagged { .. } => false,
        };
    }

    pub fn line_repair_fill_push_in_group_end(curr: LineCandidate, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: SortedSetTable<u32>, unbreakable_ranges: UnbreakableRanges) -> Option<u32> {
        let mut group_end = curr.cluster_range.start;
        while (i32::from_ne_bytes(((group_end) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((curr.cluster_range.end) as i32).to_ne_bytes()) {
            let containing = unbreakable_ranges.containing_from_closed_start_or_null(group_end);
            match &(containing) {
                Some(__option12) => {
                    group_end = __option12.end;
                    if i32::from_ne_bytes(((group_end) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((curr.cluster_range.end) as i32).to_ne_bytes())) {
                        return None;
                    }
                    continue;
                }
                None => {
                }
            }
            if forbidden_line_end_clusters.has(&(group_end)) {
                group_end = u32::wrapping_add(group_end, 1);
                continue;
            }
            let next_head = u32::wrapping_add(group_end, 1);
            if i32::from_ne_bytes(((next_head) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((curr.cluster_range.end) as i32).to_ne_bytes()) && forbidden_line_start_clusters.is_some() && (forbidden_line_start_clusters).as_ref().unwrap().has(&(next_head)) {
                group_end = next_head;
                continue;
            }
            return Some(group_end);
        }
        return None;
    }

    pub fn line_repair_apply_fill_push_in(lines: &Vec<LineCandidate>, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: &Vec<ShrinkOpportunity>, first_line_indent: f64, compress_bias: f64, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: SortedSetTable<u32>, unbreakable_ranges: UnbreakableRanges, push_in_penalty: u32, gap_boundaries: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<Vec<LineCandidate>, TextRangeError> {
        if i32::from_ne_bytes(((u32::try_from((lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) < (2) || (compress_bias) <= 0 as f64 {
            return Ok((*lines).clone());
        }
        let gaps = match &(gap_boundaries) { None => LineBreakerLines::line_breaker_lines_empty_int_set(), Some(__option13) => (*__option13).clone() };
        let progressive = match &(progressive_break_opportunities) { None => LineBreakerLines::line_breaker_lines_empty_progressive_map(), Some(__option14) => (*__option14).clone() };
        let mut out = lines.clone();
        let mut i = 0;
        while (i) < (i32::wrapping_sub(i32::from_ne_bytes(((i32::from_ne_bytes(u32::try_from(((out).len()) & 4294967295).unwrap_or(0).to_ne_bytes())) as i32).to_ne_bytes()), 1)) {
            let prev = (out[usize::try_from(i).unwrap_or(0)]).clone();
            let curr = (out[usize::try_from(i32::wrapping_add(i, 1)).unwrap_or(0)]).clone();
            let can_extend_zero_shrink_fill = LineRepair::line_repair_is_continuable_zero_shrink_fill_push_in((prev.repair).clone());
            if match &(prev.repair) { Some(__option15) => !can_extend_zero_shrink_fill, None => false } || prev.get_hanging_cluster_index().is_some() || prev.end_reason != LineEndReason::AutoWrap {
                i = i32::wrapping_add(i, 1);
                continue;
            }
            let limit = ProgressiveBreakDecisions::progressive_break_decisions_line_limit(max_width, first_line_indent, prev.cluster_range.start);
            let deficit = limit - prev.adjusted_width;
            if deficit <= 0 as f64 {
                i = i32::wrapping_add(i, 1);
                continue;
            }
            let curr0 = curr.cluster_range.start;
            let mut group_end = LineRepair::line_repair_fill_push_in_group_end((curr).clone(), (forbidden_line_start_clusters).clone(), (forbidden_line_end_clusters).clone(), (unbreakable_ranges).clone());
            if group_end.is_none() {
                i = i32::wrapping_add(i, 1);
                continue;
            }
            let current_break = progressive.get(&(u32::wrapping_add(prev.cluster_range.end, 1)));
            let mut resulting_break = progressive.get(&(u32::wrapping_add(group_end.unwrap_or(0), 1)));
            let mut added_terms: Vec<f64> = vec![];
            let mut c_idx = curr0;
            while (i32::from_ne_bytes(((c_idx) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((group_end.unwrap_or(0)) as i32).to_ne_bytes()) {
                added_terms.push(adjusted_clusters[usize::try_from(c_idx).unwrap_or(0)].advance);
                c_idx = u32::wrapping_add(c_idx, 1);
            }
            let mut added_advance = AccurateSum::accurate_sum_of(&added_terms);
            let mut promotes_progressive_tier = match &(current_break) { Some(__option16) => resulting_break.is_some() && __option16.span_range.start == ((resulting_break).as_ref().unwrap().span_range).clone().start && __option16.span_range.end == ((resulting_break).as_ref().unwrap().span_range).clone().end && (i32::from_ne_bytes(((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((resulting_break).as_ref().unwrap().tier)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(__option16.tier)) as i32).to_ne_bytes())), None => false };
            if promotes_progressive_tier && (added_advance) < (deficit - 0.001f64) {
                let active_break = (current_break).clone();
                let search_start = u32::wrapping_add(group_end.unwrap_or(0), 2);
                let search_end = u32::wrapping_add(curr.cluster_range.end, 1);
                let mut matching_tier_boundary: Option<u32> = None;
                if i32::from_ne_bytes(((search_start) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((search_end) as i32).to_ne_bytes()) {
                    let mut boundary = search_start;
                    while (i32::from_ne_bytes(((boundary) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((search_end) as i32).to_ne_bytes()) {
                        let opp = progressive.get(&(boundary));
                        if match &(opp) { Some(__option17) => __option17.span_range.start == (active_break.as_ref().unwrap().span_range).clone().start && __option17.span_range.end == (active_break.as_ref().unwrap().span_range).clone().end && __option17.tier == active_break.as_ref().unwrap().tier, None => false } {
                            matching_tier_boundary = Some(boundary);
                            break;
                        }
                        boundary = u32::wrapping_add(boundary, 1);
                    }
                }
                match &(matching_tier_boundary) {
                    Some(__option18) => {
                        group_end = Some(u32::wrapping_sub(*__option18, 1));
                        resulting_break = progressive.get(&(*__option18));
                        let mut reassigned_terms: Vec<f64> = vec![];
                        let mut c_idx2 = curr0;
                        while (i32::from_ne_bytes(((c_idx2) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((group_end.unwrap_or(0)) as i32).to_ne_bytes()) {
                            reassigned_terms.push(adjusted_clusters[usize::try_from(c_idx2).unwrap_or(0)].advance);
                            c_idx2 = u32::wrapping_add(c_idx2, 1);
                        }
                        added_advance = AccurateSum::accurate_sum_of(&reassigned_terms);
                        promotes_progressive_tier = false;
                    }
                    None => {
                    }
                }
            }
            if match &(current_break) { Some(__option19) => resulting_break.is_some() && __option19.span_range.start == ((resulting_break).as_ref().unwrap().span_range).clone().start && __option19.span_range.end == ((resulting_break).as_ref().unwrap().span_range).clone().end && (i32::from_ne_bytes(((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((resulting_break).as_ref().unwrap().tier)) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(__option19.tier)) as i32).to_ne_bytes())), None => false } {
                i = i32::wrapping_add(i, 1);
                continue;
            }
            let overflow = added_advance - deficit;
            if promotes_progressive_tier && (overflow) < (-0.001f64) {
                i = i32::wrapping_add(i, 1);
                continue;
            }
            if overflow >= deficit * compress_bias {
                i = i32::wrapping_add(i, 1);
                continue;
            }
            if overflow > (0.0f64) && !promotes_progressive_tier {
                let prev_gaps = LineBreakerLines::line_breaker_lines_line_gap_count((prev.cluster_range).clone(), (gaps).clone());
                let d_stretch_cured = if prev_gaps == 0 { 0.0f64 } else { deficit / format!("{}", (i32::from_ne_bytes(((prev_gaps) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) };
                let group_gaps = LineBreakerLines::line_breaker_lines_line_gap_count(IntRange::new(prev.cluster_range.start, group_end.unwrap_or(0)), (gaps).clone());
                let d_compression_introduced = overflow / format!("{}", { let v: u32 = if i32::from_ne_bytes(((group_gaps) as i32).to_ne_bytes()) > (1) { group_gaps } else { 1 }; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0);
                if d_compression_introduced > (d_stretch_cured) {
                    i = i32::wrapping_add(i, 1);
                    continue;
                }
            }
            let result = LineRepair::line_repair_try_push_in((prev).clone(), (curr).clone(), &natural_clusters, &adjusted_clusters, limit, &shrink_opportunities, push_in_penalty, group_end, if promotes_progressive_tier { Some(UString::from("ProgressiveTechnicalTierPromotion")) } else { Some(UString::from("LineAdjustmentPushIn")) }.clone())?;
            if result.candidate.clone().accepted {
                out[usize::try_from(i).unwrap_or(0)] = (result.previous).clone();
                match &(result.current) {
                    None => {
                        { let _a = &mut (out); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes(((i32::wrapping_add(i, 1)) as i32).to_ne_bytes()); let splice_index2 = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos }; let _len = 1i32; let splice_count2 = if _len < 0 { 0 } else { let _rest = _n - splice_index2; if _len < _rest { _len } else { _rest } }; let splice_removed2: Vec<_> = _a.drain(usize::try_from(splice_index2).unwrap_or(0)..usize::try_from(splice_index2 + splice_count2).unwrap_or(0)).collect(); splice_removed2 };
                    }
                    Some(__option20) => {
                        out[usize::try_from(i32::wrapping_add(i, 1)).unwrap_or(0)] = (*__option20).clone();
                    }
                }
                if LineRepair::line_repair_is_continuable_zero_shrink_fill_push_in(((result.previous).clone().repair).clone()) && result.current.is_some() {
                    continue;
                }
            }
            i = i32::wrapping_add(i, 1);
        }
        return Ok(out);
    }

    pub fn line_repair_with_fill_push_in(solution: LineSolution, enabled: bool, natural_clusters: &Vec<Cluster>, adjusted_clusters: &Vec<Cluster>, max_width: f64, shrink_opportunities: &Vec<ShrinkOpportunity>, first_line_indent: f64, compress_bias: f64, forbidden_line_start_clusters: Option<SortedSetTable<u32>>, forbidden_line_end_clusters: SortedSetTable<u32>, unbreakable_ranges: UnbreakableRanges, push_in_penalty: u32, gap_boundaries: Option<SortedSetTable<u32>>, progressive_break_opportunities: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        if !enabled {
            return Ok(solution);
        }
        return Ok(LineSolution::new(Some(LineRepair::line_repair_apply_fill_push_in(&solution.lines, &natural_clusters, &adjusted_clusters, max_width, &shrink_opportunities, first_line_indent, compress_bias, (forbidden_line_start_clusters).clone(), (forbidden_line_end_clusters).clone(), (unbreakable_ranges).clone(), push_in_penalty, (gap_boundaries).clone(), (progressive_break_opportunities).clone())?), Some(solution.total_badness))?);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShrinkTierEntry {
    pub key: u32,
    pub value: ShrinkOpportunity,
}
