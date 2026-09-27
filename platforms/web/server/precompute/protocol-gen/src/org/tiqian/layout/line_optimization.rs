use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::linebreak::break_kind::BreakKind;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub struct BreakCandidate {
    pub index: u32,
    pub kind: BreakKind,
    pub natural_width: f64,
    pub compressed_width: f64,
    pub expanded_width: f64,
    pub forbidden_reason: Option<String>,
    pub repair_options: Vec<RepairOption>,
}

impl BreakCandidate {
    pub fn new(index: u32, kind: BreakKind, natural_width: f64, compressed_width: f64, expanded_width: f64, forbidden_reason: Option<String>, repair_options: Option<Vec<RepairOption>>) -> Result<Self, TextRangeError> {
        let repair_options = repair_options.unwrap_or_else(|| vec![]);
        Ok(Self {
            index,
            kind,
            natural_width,
            compressed_width,
            expanded_width,
            forbidden_reason: match forbidden_reason { Some(v) => Some(v.to_string()), None => None },
            repair_options: repair_options,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "BreakCandidate(",
            "index=",
            crate::runtime::int_text::IntText::int_text(self.index),
            ", ",
            "kind=",
            self.kind.name(),
            ", ",
            "naturalWidth=",
            self.natural_width,
            ", ",
            "compressedWidth=",
            self.compressed_width,
            ", ",
            "expandedWidth=",
            self.expanded_width,
            ", ",
            "forbiddenReason=",
            match (self.forbidden_reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "repairOptions=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.repair_options).clone();
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
pub struct RepairOptions;

impl RepairOptions {
    pub fn repair_options_penalty(o: RepairOption) -> u32 {
        return match o {
            RepairOption::PushIn { penalty: _p0, .. } => _p0,
            RepairOption::Hang { penalty: _p0, .. } => _p0,
            RepairOption::CarryPrevious { penalty: _p0, .. } => _p0,
            RepairOption::CarryNext { penalty: _p0, .. } => _p0,
            RepairOption::LeaveRagged { penalty: _p0, .. } => _p0,
        };
    }

    pub fn repair_options_reason(o: RepairOption) -> String {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::Hang { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::CarryPrevious { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::CarryNext { penalty: _p0, reason: _p1, .. } => _p1,
            RepairOption::LeaveRagged { penalty: _p0, reason: _p1, .. } => _p1,
        };
    }

    pub fn repair_options_hang_offender(o: RepairOption) -> Option<u32> {
        return match o {
            RepairOption::PushIn { .. } => None,
            RepairOption::Hang { penalty: _p0, reason: _p1, offender_cluster_index: _p2 } => Some(_p2),
            RepairOption::CarryPrevious { .. } => None,
            RepairOption::CarryNext { .. } => None,
            RepairOption::LeaveRagged { .. } => None,
        };
    }

    pub fn repair_options_push_in_reason_of(o: RepairOption) -> Option<String> {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, .. } => Some(_p1),
            RepairOption::Hang { .. } => None,
            RepairOption::CarryPrevious { .. } => None,
            RepairOption::CarryNext { .. } => None,
            RepairOption::LeaveRagged { .. } => None,
        };
    }

    pub fn repair_options_push_in_allocations(o: RepairOption) -> Option<Vec<PushInAllocation>> {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, .. } => Some(_p3),
            RepairOption::Hang { .. } => None,
            RepairOption::CarryPrevious { .. } => None,
            RepairOption::CarryNext { .. } => None,
            RepairOption::LeaveRagged { .. } => None,
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PushInAllocation {
    pub cluster_index: u32,
    pub shrink: f64,
    pub available_capacity: f64,
    pub channel: ShrinkChannel,
}

impl PushInAllocation {
    pub fn new(cluster_index: u32, shrink: f64, available_capacity: f64, channel: Option<ShrinkChannel>) -> Result<Self, TextRangeError> {
        let channel = channel.unwrap_or_else(|| ShrinkChannel::TrailingGlue);
        Ok(Self {
            cluster_index,
            shrink,
            available_capacity,
            channel: channel,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "PushInAllocation(",
            "clusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.cluster_index),
            ", ",
            "shrink=",
            self.shrink,
            ", ",
            "availableCapacity=",
            self.available_capacity,
            ", ",
            "channel=",
            self.channel.name(),
            ")"
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct LineCandidate {
    pub cluster_range: IntRange,
    pub source_range: TextRange,
    pub natural_width: f64,
    pub adjusted_width: f64,
    pub end_reason: LineEndReason,
    pub repair: Option<RepairOption>,
    pub repair_candidates: Vec<RepairCandidate>,
    pub hanging_cluster_indices: SortedSetTable<u32>,
}

impl LineCandidate {
    pub fn new(cluster_range: IntRange, source_range: TextRange, natural_width: f64, adjusted_width: f64, end_reason: Option<LineEndReason>, repair: Option<RepairOption>, repair_candidates: Option<Vec<RepairCandidate>>, hanging_cluster_indices: Option<SortedSetTable<u32>>) ->
Result<Self, TextRangeError> {
        let end_reason = end_reason.unwrap_or_else(|| LineEndReason::AutoWrap);
        let repair_candidates = repair_candidates.unwrap_or_else(|| vec![]);
        let hanging_cluster_indices = hanging_cluster_indices.unwrap_or_else(|| LineCandidate::line_candidate_empty_hanging());
        if i32::from_ne_bytes((u32::from_ne_bytes((hanging_cluster_indices.size()).to_ne_bytes())).to_ne_bytes()) > (0) {
            let first_hanging = hanging_cluster_indices.at(0i32);
            let last_hanging = hanging_cluster_indices.at(i32::from_ne_bytes((u32::wrapping_sub(u32::from_ne_bytes((hanging_cluster_indices.size()).to_ne_bytes()), 1)).to_ne_bytes()));
            if !((i32::from_ne_bytes((cluster_range.start).to_ne_bytes())) <= i32::from_ne_bytes((first_hanging).to_ne_bytes()) && (i32::from_ne_bytes((first_hanging).to_ne_bytes())) <= i32::from_ne_bytes((cluster_range.end).to_ne_bytes()) && last_hanging == cluster_range.end) {
                return Err(TextRangeError::Message { text: format!("{}{}{}{}",
            "Hanging clusters must be a trailing line suffix: line=",
            LineCandidates::line_candidates_render_range((cluster_range).clone()),
            " hanging=",
            {
        let mut out = String::new();
        out.push('[');
        let set = hanging_cluster_indices;
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    }
        ).to_string() });
            }
            if u32::from_ne_bytes((hanging_cluster_indices.size()).to_ne_bytes()) != u32::wrapping_add(u32::wrapping_sub(cluster_range.end, first_hanging), 1) {
                return Err(TextRangeError::Message { text: format!("{}{}{}{}",
            "Hanging clusters must be contiguous: line=",
            LineCandidates::line_candidates_render_range((cluster_range).clone()),
            " hanging=",
            {
        let mut out = String::new();
        out.push('[');
        let set = hanging_cluster_indices;
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    }
        ).to_string() });
            }
        }
        Ok(Self {
            cluster_range,
            source_range,
            natural_width,
            adjusted_width,
            end_reason: end_reason,
            repair,
            repair_candidates: repair_candidates,
            hanging_cluster_indices: hanging_cluster_indices,
        })
    }

    pub fn get_hanging_cluster_index(&self) -> Option<u32> {
        let from_repair = match &(self.repair) { None => None, Some(__option) => RepairOptions::repair_options_hang_offender((*__option).clone()) };
        let last = if u32::from_ne_bytes(((self.hanging_cluster_indices).clone().size()).to_ne_bytes()) == 0 { None } else {
Some((self.hanging_cluster_indices).clone().at(i32::from_ne_bytes((u32::wrapping_sub(u32::from_ne_bytes(((self.hanging_cluster_indices).clone().size()).to_ne_bytes()), 1)).to_ne_bytes()))) };
        return match &(from_repair) { Some(__option1) => Some(*__option1), None => last };
    }

    pub fn get_in_measure_cluster_range(&self) -> IntRange {
        let first_hanging = if u32::from_ne_bytes(((self.hanging_cluster_indices).clone().size()).to_ne_bytes()) == 0 { None } else { Some((self.hanging_cluster_indices).clone().at(0i32)) };
        return match &(first_hanging) { None => (self.cluster_range).clone(), Some(__option2) => IntRange::new((self.cluster_range).clone().start, u32::wrapping_sub(*__option2, 1)) };
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineCandidate(",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "sourceRange=",
            (self.source_range).clone().to_string(),
            ", ",
            "naturalWidth=",
            self.natural_width,
            ", ",
            "adjustedWidth=",
            self.adjusted_width,
            ", ",
            "endReason=",
            self.end_reason.name(),
            ", ",
            "repair=",
            (match &(self.repair) { None => "null".to_string(), Some(__option3) => (*__option3).clone().to_string() }),
            ", ",
            "repairCandidates=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.repair_candidates).clone();
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
            "hangingClusterIndices=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.hanging_cluster_indices).clone();
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
            ")"
        );
    }

    pub fn line_candidate_empty_hanging() -> SortedSetTable<u32> {
        let b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        return b.clone().build();
    }
}

#[derive(Clone, Copy)]
pub struct LineCandidates;

impl LineCandidates {
    pub fn line_candidates_render_range(r: IntRange) -> String {
        return format!("{}{}{}",
            crate::runtime::int_text::IntText::int_text(r.start),
            "..",
            crate::runtime::int_text::IntText::int_text(r.end)
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepairCandidate {
    pub kind: String,
    pub reason_code: String,
    pub offender_cluster_index: u32,
    pub penalty: u32,
    pub accepted: bool,
    pub rejection_reason: Option<String>,
    pub target_cluster_index: Option<u32>,
    pub carried_cluster_index: Option<u32>,
    pub shrink: f64,
    pub required_shrink: f64,
    pub available_capacity: f64,
}

impl RepairCandidate {
    pub fn new(kind: &str, reason_code: &str, offender_cluster_index: u32, penalty: u32, accepted: bool, rejection_reason: Option<String>, target_cluster_index: Option<u32>, carried_cluster_index: Option<u32>, shrink: Option<f64>, required_shrink: Option<f64>, available_capacity:
Option<f64>) -> Result<Self, TextRangeError> {
        let shrink = shrink.unwrap_or_else(|| 0 as f64);
        let required_shrink = required_shrink.unwrap_or_else(|| 0 as f64);
        let available_capacity = available_capacity.unwrap_or_else(|| 0 as f64);
        Ok(Self {
            kind: kind.to_string(),
            reason_code: reason_code.to_string(),
            offender_cluster_index,
            penalty,
            accepted,
            rejection_reason: match rejection_reason { Some(v) => Some(v.to_string()), None => None },
            target_cluster_index,
            carried_cluster_index,
            shrink: shrink,
            required_shrink: required_shrink,
            available_capacity: available_capacity,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RepairCandidate(",
            "kind=",
            (self.kind).to_string(),
            ", ",
            "reasonCode=",
            (self.reason_code).to_string(),
            ", ",
            "offenderClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.offender_cluster_index),
            ", ",
            "penalty=",
            crate::runtime::int_text::IntText::int_text(self.penalty),
            ", ",
            "accepted=",
            self.accepted,
            ", ",
            "rejectionReason=",
            match (self.rejection_reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "targetClusterIndex=",
            match self.target_cluster_index { Some(v) => crate::runtime::int_text::IntText::int_text(v), None => "null".to_string() },
            ", ",
            "carriedClusterIndex=",
            match self.carried_cluster_index { Some(v) => crate::runtime::int_text::IntText::int_text(v), None => "null".to_string() },
            ", ",
            "shrink=",
            self.shrink,
            ", ",
            "requiredShrink=",
            self.required_shrink,
            ", ",
            "availableCapacity=",
            self.available_capacity,
            ")"
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct LineSolution {
    pub lines: Vec<LineCandidate>,
    pub total_badness: f64,
}

impl LineSolution {
    pub fn new(lines: Option<Vec<LineCandidate>>, total_badness: Option<f64>) -> Result<Self, TextRangeError> {
        let lines = lines.unwrap_or_else(|| vec![]);
        let total_badness = total_badness.unwrap_or_else(|| 0 as f64);
        Ok(Self {
            lines: lines,
            total_badness: total_badness,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "LineSolution(",
            "lines=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.lines).clone();
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
            "totalBadness=",
            self.total_badness,
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepairOption {
    PushIn { penalty: u32, reason: String, offender_cluster_index: u32, allocations: Vec<PushInAllocation>, total_shrink: f64, total_available_capacity: f64 },
    Hang { penalty: u32, reason: String, offender_cluster_index: u32 },
    CarryPrevious { penalty: u32, reason: String, offender_cluster_index: u32, carried_cluster_index: u32 },
    CarryNext { penalty: u32, reason: String, moved_cluster_index: u32 },
    LeaveRagged { penalty: u32, reason: String, offender_cluster_index: u32 },
}

impl RepairOption {
    pub fn to_string(&self) -> String {
        match self {
            RepairOption::PushIn { penalty, reason, offender_cluster_index, allocations, total_shrink, total_available_capacity } => format!("PushIn(penalty={}, reason={}, offender_cluster_index={}, allocations={}, total_shrink={}, total_available_capacity={})",
(penalty).to_string(), (reason).clone(), (offender_cluster_index).to_string(), {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < allocations.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", (allocations[j0]).to_string());
                j0 += 1;
            }
            out.push(']');
            out
        }, (total_shrink).to_string(), (total_available_capacity).to_string()),
            RepairOption::Hang { penalty, reason, offender_cluster_index } => format!("Hang(penalty={}, reason={}, offender_cluster_index={})", (penalty).to_string(), (reason).clone(), (offender_cluster_index).to_string()),
            RepairOption::CarryPrevious { penalty, reason, offender_cluster_index, carried_cluster_index } => format!("CarryPrevious(penalty={}, reason={}, offender_cluster_index={}, carried_cluster_index={})", (penalty).to_string(), (reason).clone(),
(offender_cluster_index).to_string(), (carried_cluster_index).to_string()),
            RepairOption::CarryNext { penalty, reason, moved_cluster_index } => format!("CarryNext(penalty={}, reason={}, moved_cluster_index={})", (penalty).to_string(), (reason).clone(), (moved_cluster_index).to_string()),
            RepairOption::LeaveRagged { penalty, reason, offender_cluster_index } => format!("LeaveRagged(penalty={}, reason={}, offender_cluster_index={})", (penalty).to_string(), (reason).clone(), (offender_cluster_index).to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineOptimizationStrategy {
    Greedy,
    Lookahead,
    ParagraphDynamicProgramming,
}

pub fn compare_line_optimization_strategy(a: &LineOptimizationStrategy, b: &LineOptimizationStrategy) -> i32 {
    if a == b { return 0; }
    fn rank(v: &LineOptimizationStrategy) -> i32 {
        match v {
            LineOptimizationStrategy::Greedy => 0,
            LineOptimizationStrategy::Lookahead => 1,
            LineOptimizationStrategy::ParagraphDynamicProgramming => 2,
        }
    }
    rank(a) - rank(b)
}

impl LineOptimizationStrategy {
    pub fn to_string(&self) -> String {
        match self {
            LineOptimizationStrategy::Greedy => "Greedy".to_string(),
            LineOptimizationStrategy::Lookahead => "Lookahead".to_string(),
            LineOptimizationStrategy::ParagraphDynamicProgramming => "ParagraphDynamicProgramming".to_string(),
        }
    }
}
