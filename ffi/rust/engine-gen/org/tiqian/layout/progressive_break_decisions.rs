use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::LazyLock;


#[derive(Debug, Clone, PartialEq)]
pub struct ProgressiveBreakOpportunity {
    pub tier: ProgressiveBreakTier,
    pub span_range: TextRange,
    pub preceding_whitespace_stretch_capacity: f64,
}

impl ProgressiveBreakOpportunity {
    pub fn new(tier: ProgressiveBreakTier, span_range: TextRange, preceding_whitespace_stretch_capacity: Option<f64>) -> Self {
        let preceding_whitespace_stretch_capacity = preceding_whitespace_stretch_capacity.unwrap_or_else(|| 0.0);
        Self {
            tier,
            span_range,
            preceding_whitespace_stretch_capacity: preceding_whitespace_stretch_capacity,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ProgressiveBreakOpportunity(")); __s += &(UString::from("tier=")); __s += UString::from(self.tier.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("spanRange=")); __s += UString::from(format!("{}", (self.span_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("precedingWhitespaceStretchCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.preceding_whitespace_stretch_capacity)); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, Copy)]
pub struct ProgressiveBreakDecisions;

impl ProgressiveBreakDecisions {
    pub fn progressive_break_decisions_decide_progressive_break(line_start: u32, overflow_at: u32, opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>, adjusted_clusters: Option<Vec<Cluster>>, line_limit: Option<f64>, cjk_inter_char_boundaries: Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>) -> u32 {
        let limit = match &(line_limit) { None => f64::INFINITY, Some(__option) => *__option };
        let cjk = match &(cjk_inter_char_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(),
Some(__option1) => (*__option1).clone() };
        let max = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option2) => *__option2 };
        let sino = match &(sino_western_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(),
Some(__option3) => (*__option3).clone() };
        let cap = match &(sino_western_stretch_cap) { None => 0.0f64, Some(__option4) => *__option4 };
        let active = opportunities.get(&(overflow_at));
        if active.is_none() {
            return overflow_at;
        }
        let best_priority = ProgressiveBreakDecisions::progressive_break_decisions_progressive_break_priority_for_line(line_start, overflow_at, ((active).as_ref().unwrap().clone()).clone(), (opportunities).clone(), (adjusted_clusters).clone(), limit, (cjk).clone(), max, (sino).clone(), cap);
        let mut best: Option<u32> = None;
        let mut boundary = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((boundary) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((overflow_at) as i32).to_ne_bytes()) {
            let o = opportunities.get(&(boundary));
            if match &(o) { Some(__option5) => __option5.span_range.start == ((active).as_ref().unwrap().span_range).clone().start && __option5.span_range.end == ((active).as_ref().unwrap().span_range).clone().end && ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(__option5.tier) == best_priority && (match &(best) { None => true, Some(__option6) => i32::from_ne_bytes(((boundary) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((*__option6) as i32).to_ne_bytes())) }), None => false } {
                best = Some(boundary);
            }
            boundary = u32::wrapping_add(boundary, 1);
        }
        return match &(best) { None => overflow_at, Some(__option7) => *__option7 };
    }

    pub fn progressive_break_decisions_progressive_candidate_allowed(line_start: u32, raw_greedy: u32, candidate_end: u32, opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>, adjusted_clusters: Option<Vec<Cluster>>, line_limit: Option<f64>, cjk_inter_char_boundaries: Option<SortedSetTable<u32>>, max_cjk_stretch_per_gap: Option<f64>, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>) -> bool {
        let limit = match &(line_limit) { None => f64::INFINITY, Some(__option8) => *__option8 };
        let cjk = match &(cjk_inter_char_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(),
Some(__option9) => (*__option9).clone() };
        let max = match &(max_cjk_stretch_per_gap) { None => f64::INFINITY, Some(__option10) => *__option10 };
        let sino = match &(sino_western_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(),
Some(__option11) => (*__option11).clone() };
        let cap = match &(sino_western_stretch_cap) { None => 0.0f64, Some(__option12) => *__option12 };
        let active = opportunities.get(&(raw_greedy));
        if active.is_none() {
            return true;
        }
        let candidate = opportunities.get(&(candidate_end));
        if candidate.is_none() {
            if match &(adjusted_clusters) { None => true, Some(__option13) => candidate_end > 2147483647 } || (candidate_end) >= u32::try_from(((adjusted_clusters).as_ref().map_or(0, |v| v.len())) & 0xFFFF_FFFF).unwrap_or(0) {
                return true;
            }
            let source = (((adjusted_clusters).as_ref().unwrap()[usize::try_from(candidate_end).unwrap_or(0)]).clone().range).clone().start;
            return (i32::from_ne_bytes(((source) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((((active).as_ref().unwrap().span_range).clone().start) as i32).to_ne_bytes()) || (i32::from_ne_bytes(((source) as i32).to_ne_bytes())) >= i32::from_ne_bytes(((((active).as_ref().unwrap().span_range).clone().end) as i32).to_ne_bytes());
        }
        if candidate.as_ref().unwrap().span_range.clone().start != ((active).as_ref().unwrap().span_range).clone().start || ((candidate).as_ref().unwrap().span_range).clone().end != ((active).as_ref().unwrap().span_range).clone().end {
            return true;
        }
        if i32::from_ne_bytes(((candidate_end) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((raw_greedy) as i32).to_ne_bytes())) {
            return (i32::from_ne_bytes(((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((candidate).as_ref().unwrap().tier)) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((active).as_ref().unwrap().tier)) as i32).to_ne_bytes());
        }
        let selected = ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(line_start, raw_greedy, (opportunities).clone(), (adjusted_clusters).clone(), Some(limit), Some((cjk).clone()), Some(max), Some((sino).clone()), Some(cap));
        return candidate_end == selected;
    }

    pub(crate) fn progressive_break_decisions_progressive_break_priority_for_line(line_start: u32, overflow_at: u32, active: ProgressiveBreakOpportunity, opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>, adjusted_clusters: Option<Vec<Cluster>>, line_limit: f64, cjk_inter_char_boundaries: SortedSetTable<u32>, max_cjk_stretch_per_gap: f64, sino_western_boundaries: SortedSetTable<u32>, sino_western_stretch_cap: f64) -> u32 {
        let mut priorities_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((overflow_at) as i32).to_ne_bytes()) {
            let o = opportunities.get(&(i));
            if match &(o) { Some(__option14) => __option14.span_range.start == (active.span_range).clone().start && __option14.span_range.end == (active.span_range).clone().end, None => false } {
                priorities_builder.put(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority((o).as_ref().unwrap().tier)));
            }
            i = u32::wrapping_add(i, 1);
        }
        let priorities: SortedSetTable<u32> = priorities_builder.clone().build();
        if u32::from_ne_bytes(((priorities.size()) as u32).to_ne_bytes()) == 0 {
            return ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(active.tier);
        }
        if match &(adjusted_clusters) { None => true, Some(__option15) => !(line_limit).is_finite() } || !(max_cjk_stretch_per_gap).is_finite() {
            return priorities.at(0i32);
        }
        let stretch = max_cjk_stretch_per_gap * 0.0f64;
        let mut least = priorities.at(0i32);
        let mut density = f64::INFINITY;
        let mut least_boundary = u32::wrapping_add(line_start, 1);
        let mut pi = 0u32;
        while (i32::from_ne_bytes(((pi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((priorities.size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            let priority = priorities.at(i32::from_ne_bytes(((pi) as i32).to_ne_bytes()));
            let mut b = 0u32;
            i = u32::wrapping_add(line_start, 1);
            while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((overflow_at) as i32).to_ne_bytes()) {
                let o = opportunities.get(&(i));
                if match &(o) { Some(__option16) => __option16.span_range.start == (active.span_range).clone().start && __option16.span_range.end == (active.span_range).clone().end && ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(__option16.tier) == priority, None => false } {
                    b = i;
                }
                i = u32::wrapping_add(i, 1);
            }
            if b == 0 {
                pi = u32::wrapping_add(pi, 1);
                continue;
            }
            let d = ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_stretch_density(line_start, b, (opportunities).clone(), (adjusted_clusters).as_ref().unwrap(), line_limit, (cjk_inter_char_boundaries).clone(), (sino_western_boundaries).clone(), sino_western_stretch_cap);
            if d < (density) {
                density = d;
                least = priority;
                least_boundary = b;
            }
            if d <= stretch {
                return priority;
            }
            pi = u32::wrapping_add(pi, 1);
        }
        let mut emergency = 0u32;
        i = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((overflow_at) as i32).to_ne_bytes()) {
            let o = opportunities.get(&(i));
            if match &(o) { Some(__option17) => __option17.span_range.start == (active.span_range).clone().start && __option17.span_range.end == (active.span_range).clone().end && __option17.tier == ProgressiveBreakTier::Emergency, None => false } {
                emergency = i;
            }
            i = u32::wrapping_add(i, 1);
        }
        return if emergency != 0 && (i32::from_ne_bytes(((emergency) as i32).to_ne_bytes())) >= i32::from_ne_bytes(((least_boundary) as i32).to_ne_bytes()) { ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Emergency) } else { least };
    }

    pub(crate) fn progressive_break_decisions_progressive_candidate_stretch_density(line_start: u32, boundary: u32, opportunities: SortedMapTable<u32, ProgressiveBreakOpportunity>, adjusted_clusters: &Vec<Cluster>, line_limit: f64, cjk_inter_char_boundaries: SortedSetTable<u32>, sino_western_boundaries: SortedSetTable<u32>, sino_western_stretch_cap: f64) -> f64 {
        let mut width = 0.0f64;
        let mut i = line_start;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((boundary) as i32).to_ne_bytes())) {
            width += adjusted_clusters[usize::try_from(i).unwrap_or(0)].advance;
            i = u32::wrapping_add(i, 1);
        }
        let deficit = { let __min_a = (line_limit - width) as f64; let __min_b = 0.0f64 as f64; if __min_a.is_nan() || __min_b.is_nan() { f64::NAN } else { if __min_a > __min_b { __min_a } else if __min_b > __min_a { __min_b } else if __min_a == 0.0 && __min_b == 0.0 { if __min_a.is_sign_negative() { __min_b } else { __min_a } } else { __min_a } } };
        let mut technical = 0.0f64;
        i = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((boundary) as i32).to_ne_bytes())) {
            let o = opportunities.get(&(i));
            match &(o) {
                Some(__option18) => {
                    if __option18.tier == ProgressiveBreakTier::Whitespace {
                    technical += __option18.preceding_whitespace_stretch_capacity;
                    }
                }
                None => {
                }
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut sino = 0u32;
        i = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((boundary) as i32).to_ne_bytes())) {
            if sino_western_boundaries.has(&(i)) {
                sino = u32::wrapping_add(sino, 1);
            }
            i = u32::wrapping_add(i, 1);
        }
        let cjk_deficit = { let __min_a1 = ((deficit - technical) - format!("{}", (i32::from_ne_bytes(((sino) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * sino_western_stretch_cap) as f64; let __min_b1 = 0.0f64 as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 > __min_b1 { __min_a1 } else if __min_b1 > __min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_b1 } else { __min_a1 } } else { __min_a1 } } };
        let active = opportunities.get(&(boundary));
        let mut units = 0u32;
        match &(active) {
            Some(__option19) => {
                i = line_start;
                while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((boundary) as i32).to_ne_bytes())) {
                    let c = (adjusted_clusters[usize::try_from(i).unwrap_or(0)]).clone();
                    if i32::from_ne_bytes((((c.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((__option19.span_range.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((c.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((__option19.span_range.end) as i32).to_ne_bytes()) && !ProgressiveBreakDecisions::progressive_break_decisions_has_whitespace_unit((c.text).to_ustring().as_ustr()) {
                        units = u32::wrapping_add(units, u_string::unit_count(&((c.text).to_ustring())));
                    }
                    i = u32::wrapping_add(i, 1);
                }
            }
            None => {
            }
        }
        let technical_gaps = { let __min_a2 = i32::from_ne_bytes(((u32::wrapping_sub(units, 1)) as i32).to_ne_bytes()) as f64 as f64; let __min_b2 = 0.0f64 as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 == 0.0 && __min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } };
        if technical_gaps > (0 as f64) {
            return cjk_deficit / technical_gaps;
        }
        let mut gaps = 0u32;
        i = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((boundary) as i32).to_ne_bytes())) {
            if cjk_inter_char_boundaries.has(&(i)) {
                gaps = u32::wrapping_add(gaps, 1);
            }
            i = u32::wrapping_add(i, 1);
        }
        return if gaps == 0 { cjk_deficit } else { cjk_deficit / format!("{}", (i32::from_ne_bytes(((gaps) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) };
    }

    pub(crate) fn progressive_break_decisions_has_whitespace_unit(s: &UStr) -> bool {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        let mut i = 0u32;
        let __units1 = u_string::units(&s);
        let __count1 = u_string::unit_count(&s);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) {
            let c = u_string::unit_at_from(&__units1, i).unwrap_or(0);
            if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) >= 9 && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= 13 || (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) >= 28 && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= 32 || c == 160 || c == 5760 || (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) >= 8192 && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= 8202 || c == 8232 || c == 8233 || c == 8239 || c == 8287 || c == 12288 {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }

    pub fn progressive_break_decisions_decide_hyphen_break(line_start: u32, overflow_at: u32, adjusted_clusters: &Vec<Cluster>, line_limit: f64, hyphen_break_clusters: SortedSetTable<u32>, cjk_inter_char_boundaries: SortedSetTable<u32>, max_cjk_stretch_per_gap: f64, sino_western_boundaries: Option<SortedSetTable<u32>>, sino_western_stretch_cap: Option<f64>) -> u32 {
        let sino = match &(sino_western_boundaries) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build(),
Some(__option20) => (*__option20).clone() };
        let cap = match &(sino_western_stretch_cap) { None => 0.0f64, Some(__option21) => *__option21 };
        if !hyphen_break_clusters.has(&(overflow_at)) {
            return overflow_at;
        }
        let mut whole = overflow_at;
        while (i32::from_ne_bytes(((whole) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) && hyphen_break_clusters.has(&(whole)) {
            whole = u32::wrapping_sub(whole, 1);
        }
        if i32::from_ne_bytes(((whole) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((line_start) as i32).to_ne_bytes()) {
            return overflow_at;
        }
        let mut width = 0.0f64;
        let mut k = line_start;
        while (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((whole) as i32).to_ne_bytes())) {
            width += adjusted_clusters[usize::try_from(k).unwrap_or(0)].advance;
            k = u32::wrapping_add(k, 1);
        }
        let deficit = line_limit - width;
        if deficit <= 0 as f64 {
            return whole;
        }
        let mut sw = 0u32;
        k = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((whole) as i32).to_ne_bytes())) {
            if sino.has(&(k)) {
                sw = u32::wrapping_add(sw, 1);
            }
            k = u32::wrapping_add(k, 1);
        }
        let cjk = { let __min_a3 = (deficit - format!("{}", (i32::from_ne_bytes(((sw) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * cap) as f64; let __min_b3 = 0.0f64 as f64; if __min_a3.is_nan() || __min_b3.is_nan() { f64::NAN } else { if __min_a3 > __min_b3 { __min_a3 } else if __min_b3 > __min_a3 { __min_b3 } else if __min_a3 == 0.0 && __min_b3 == 0.0 { if __min_a3.is_sign_negative() { __min_b3 } else { __min_a3 } } else { __min_a3 } } };
        let mut gaps = 0u32;
        k = u32::wrapping_add(line_start, 1);
        while (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((whole) as i32).to_ne_bytes())) {
            if cjk_inter_char_boundaries.has(&(k)) {
                gaps = u32::wrapping_add(gaps, 1);
            }
            k = u32::wrapping_add(k, 1);
        }
        return if gaps == 0 || (cjk / format!("{}", (i32::from_ne_bytes(((gaps) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0)) > (max_cjk_stretch_per_gap) { overflow_at } else { whole };
    }

    pub fn progressive_break_decisions_adjust_break_for_line_end(break_at: u32, line_start: u32, forbidden_line_end_clusters: SortedSetTable<u32>) -> u32 {
        let mut b = break_at;
        while (i32::from_ne_bytes(((u32::wrapping_sub(b, 1)) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((line_start) as i32).to_ne_bytes())) && forbidden_line_end_clusters.has(&(u32::wrapping_sub(b, 1))) {
            b = u32::wrapping_sub(b, 1);
        }
        return b;
    }

    pub fn progressive_break_decisions_line_limit(max_width: f64, first_line_indent: f64, line_start_cluster: u32) -> f64 {
        return if line_start_cluster == 0 { max_width - first_line_indent } else { max_width };
    }

    pub fn progressive_break_decisions_adjust_break_for_unbreakables(break_at: u32, line_start: u32, unbreakable_ranges: UnbreakableRanges) -> u32 {
        let mut candidate = break_at;
        loop {
            let containing = unbreakable_ranges.containing_or_null(candidate);
            if containing.is_none() {
                return candidate;
            }
            if i32::from_ne_bytes((((containing).as_ref().unwrap().start) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((line_start) as i32).to_ne_bytes()) {
                return break_at;
            }
            candidate = (containing).as_ref().unwrap().start;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShrinkOpportunity {
    pub cluster_index: u32,
    pub tier: u32,
    pub capacity: f64,
    pub channel: ShrinkChannel,
    pub line_end_only: bool,
}

impl ShrinkOpportunity {
    pub fn new(cluster_index: u32, tier: u32, capacity: f64, channel: ShrinkChannel, line_end_only: Option<bool>) -> Self {
        let line_end_only = line_end_only.unwrap_or_else(|| false);
        Self {
            cluster_index,
            tier,
            capacity,
            channel,
            line_end_only: line_end_only,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ShrinkOpportunity(")); __s += &(UString::from("clusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.cluster_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("tier=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.tier)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("capacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.capacity)); __s += &(UString::from(", ")); __s += &(UString::from("channel=")); __s += UString::from(self.channel.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineEndOnly=")); __s += UString::from(format!("{}", (self.line_end_only).to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub static UNBREAKABLE_RANGES_EMPTY: LazyLock<UnbreakableRanges> = LazyLock::new(|| UnbreakableRanges::new(vec![].to_vec()));

#[derive(Clone, PartialEq)]
pub struct UnbreakableRanges {
    pub ranges: Vec<IntRange>,
    pub(crate) by_start: Vec<IntRange>,
    pub(crate) starts_sorted: Vec<u32>,
    pub(crate) prefix_max_last: Vec<u32>,
}

impl UnbreakableRanges {
    pub fn new(ranges: Vec<IntRange>) -> Self {
    let mut by_start = ranges.clone();
    let mut starts_sorted = vec![];
    let mut prefix_max_last = vec![];
        let mut s0 = 1u32;
        while (i32::from_ne_bytes(((s0) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((by_start.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let key = (by_start[usize::try_from(s0).unwrap_or(0)]).clone();
            let mut s1 = s0;
            while (i32::from_ne_bytes(((s1) as i32).to_ne_bytes())) > (0) && (i32::from_ne_bytes(((by_start[usize::try_from(u32::wrapping_sub(s1, 1)).unwrap_or(0)].start) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((key.start) as i32).to_ne_bytes())) {
                by_start[usize::try_from(s1).unwrap_or(0)] = (by_start[usize::try_from(u32::wrapping_sub(s1, 1)).unwrap_or(0)]).clone();
                s1 = u32::wrapping_sub(s1, 1);
            }
            by_start[usize::try_from(s1).unwrap_or(0)] = key;
            s0 = u32::wrapping_add(s0, 1);
        }
        let mut si = 0u32;
        while (i32::from_ne_bytes(((si) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((by_start.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            starts_sorted.push(by_start[usize::try_from(si).unwrap_or(0)].start);
            si = u32::wrapping_add(si, 1);
        }
        let mut running = 2147483648u32;
        let mut sj = 0u32;
        while (i32::from_ne_bytes(((sj) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((by_start.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            running = u32::from_ne_bytes(((match f64::from({ let __min_a5 = i32::from_ne_bytes(((running) as i32).to_ne_bytes()) as f64 as f64; let __min_b5 = i32::from_ne_bytes(((by_start[usize::try_from(sj).unwrap_or(0)].end) as i32).to_ne_bytes()) as f64 as f64; if __min_a5.is_nan() || __min_b5.is_nan() { f64::NAN } else { if __min_a5 > __min_b5 { __min_a5 } else if __min_b5 > __min_a5 { __min_b5 } else if __min_a5 == 0.0 && __min_b5 == 0.0 { if __min_a5.is_sign_negative() { __min_b5 } else { __min_a5 } } else { __min_a5 } } }) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes());
            prefix_max_last.push(running);
            sj = u32::wrapping_add(sj, 1);
        }
        Self {
            ranges,
            by_start: by_start,
            starts_sorted: starts_sorted,
            prefix_max_last: prefix_max_last,
        }
    }

    pub fn contains_boundary(&self, candidate: u32) -> bool {
        let mut low = 0u32;
        let mut high = u32::try_from(((self.starts_sorted).clone().len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((high) as i32).to_ne_bytes())) {
            let mid = u32::wrapping_add(low, high) >> 1;
            if ({ let v: u32 = self.starts_sorted[usize::try_from(mid).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((candidate) as i32).to_ne_bytes())) {
                low = u32::wrapping_add(mid, 1);
            } else {
                high = mid;
            }
        }
        return (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) > (0) && ({ let v: u32 = self.prefix_max_last[usize::try_from(u32::wrapping_sub(low, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) >= i32::from_ne_bytes(((candidate) as i32).to_ne_bytes());
    }

    pub fn containing_or_null(&self, candidate: u32) -> Option<IntRange> {
        if !self.contains_boundary(candidate) {
            return None;
        }
        let mut ri = 0u32;
        while (i32::from_ne_bytes(((ri) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((self.ranges).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let r = (self.ranges[usize::try_from(ri).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((candidate) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((r.start) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((candidate) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((r.end) as i32).to_ne_bytes()) {
                return Some(r);
            }
            ri = u32::wrapping_add(ri, 1);
        }
        return None;
    }

    pub fn containing_from_closed_start_or_null(&self, index: u32) -> Option<IntRange> {
        let mut low = 0u32;
        let mut high = u32::try_from(((self.starts_sorted).clone().len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((high) as i32).to_ne_bytes())) {
            let mid = u32::wrapping_add(low, high) >> 1;
            if ({ let v: u32 = self.starts_sorted[usize::try_from(mid).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) <= i32::from_ne_bytes(((index) as i32).to_ne_bytes()) {
                low = u32::wrapping_add(mid, 1);
            } else {
                high = mid;
            }
        }
        if low == 0 || ({ let v: u32 = self.prefix_max_last[usize::try_from(u32::wrapping_sub(low, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) <= i32::from_ne_bytes(((index) as i32).to_ne_bytes()) {
            return None;
        }
        let mut rj = 0u32;
        while (i32::from_ne_bytes(((rj) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((self.ranges).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let r = (self.ranges[usize::try_from(rj).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((r.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((r.end) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((r.end) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) {
                return Some(r);
            }
            rj = u32::wrapping_add(rj, 1);
        }
        return None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressiveBreakTier {
    Whitespace,
    Structural,
    Syllable,
    WholeToken,
    Emergency,
}

pub fn compare_progressive_break_tier(a: &ProgressiveBreakTier, b: &ProgressiveBreakTier) -> i32 {
    if a == b { return 0; }
    fn rank(v: &ProgressiveBreakTier) -> i32 {
        match v {
            ProgressiveBreakTier::Whitespace => 0,
            ProgressiveBreakTier::Structural => 1,
            ProgressiveBreakTier::Syllable => 2,
            ProgressiveBreakTier::WholeToken => 3,
            ProgressiveBreakTier::Emergency => 4,
        }
    }
    rank(a) - rank(b)
}

impl ProgressiveBreakTier {
    pub fn to_string(&self) -> String {
        match self {
            ProgressiveBreakTier::Whitespace => "Whitespace".to_string(),
            ProgressiveBreakTier::Structural => "Structural".to_string(),
            ProgressiveBreakTier::Syllable => "Syllable".to_string(),
            ProgressiveBreakTier::WholeToken => "WholeToken".to_string(),
            ProgressiveBreakTier::Emergency => "Emergency".to_string(),
        }
    }
}

impl ProgressiveBreakTier {
    pub fn name(&self) -> &'static str {
        match self {
            ProgressiveBreakTier::Whitespace => "Whitespace",
            ProgressiveBreakTier::Structural => "Structural",
            ProgressiveBreakTier::Syllable => "Syllable",
            ProgressiveBreakTier::WholeToken => "WholeToken",
            ProgressiveBreakTier::Emergency => "Emergency",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShrinkChannel {
    TrailingGlue,
    LeadingGlue,
    LeadingAndTrailingGlue,
    RawAdvance,
}

pub fn compare_shrink_channel(a: &ShrinkChannel, b: &ShrinkChannel) -> i32 {
    if a == b { return 0; }
    fn rank(v: &ShrinkChannel) -> i32 {
        match v {
            ShrinkChannel::TrailingGlue => 0,
            ShrinkChannel::LeadingGlue => 1,
            ShrinkChannel::LeadingAndTrailingGlue => 2,
            ShrinkChannel::RawAdvance => 3,
        }
    }
    rank(a) - rank(b)
}

impl ShrinkChannel {
    pub fn to_string(&self) -> String {
        match self {
            ShrinkChannel::TrailingGlue => "TrailingGlue".to_string(),
            ShrinkChannel::LeadingGlue => "LeadingGlue".to_string(),
            ShrinkChannel::LeadingAndTrailingGlue => "LeadingAndTrailingGlue".to_string(),
            ShrinkChannel::RawAdvance => "RawAdvance".to_string(),
        }
    }
}

impl ShrinkChannel {
    pub fn name(&self) -> &'static str {
        match self {
            ShrinkChannel::TrailingGlue => "TrailingGlue",
            ShrinkChannel::LeadingGlue => "LeadingGlue",
            ShrinkChannel::LeadingAndTrailingGlue => "LeadingAndTrailingGlue",
            ShrinkChannel::RawAdvance => "RawAdvance",
        }
    }
}
