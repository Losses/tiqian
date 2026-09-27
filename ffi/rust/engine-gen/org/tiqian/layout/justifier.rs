use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::line_repair::ShrinkTierEntry;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::org::tiqian::layout::punctuation_model::GlueKind;
use crate::runtime::functional::Functional;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::string_tools;
use crate::runtime::u_string;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub struct JustificationOpportunity {
    pub target_cluster_index: u32,
    pub kind: GlueKind,
    pub priority: u32,
    pub capacity: f64,
    pub reason: Option<String>,
}

impl JustificationOpportunity {
    pub fn new(target_cluster_index: u32, kind: GlueKind, priority: u32, capacity: f64, reason: Option<String>) -> Self {
        Self {
            target_cluster_index,
            kind,
            priority,
            capacity,
            reason: match reason { Some(v) => Some(v.to_string()), None => None },
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "JustificationOpportunity(",
            "targetClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.target_cluster_index),
            ", ",
            "kind=",
            self.kind.name(),
            ", ",
            "priority=",
            crate::runtime::int_text::IntText::int_text(self.priority),
            ", ",
            "capacity=",
            self.capacity,
            ", ",
            "reason=",
            match (self.reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JustificationAllocation {
    pub target_cluster_index: u32,
    pub kind: GlueKind,
    pub priority: u32,
    pub delta: f64,
    pub reason: String,
}

impl JustificationAllocation {
    pub fn new(target_cluster_index: u32, kind: GlueKind, priority: u32, delta: f64, reason: &str) -> Self {
        Self {
            target_cluster_index,
            kind,
            priority,
            delta,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "JustificationAllocation(",
            "targetClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.target_cluster_index),
            ", ",
            "kind=",
            self.kind.name(),
            ", ",
            "priority=",
            crate::runtime::int_text::IntText::int_text(self.priority),
            ", ",
            "delta=",
            self.delta,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct JustificationPlan {
    pub line_cluster_range: IntRange,
    pub allocations: Vec<JustificationAllocation>,
    pub deficit_before: f64,
    pub unfilled_deficit: f64,
    pub fallback_reason: Option<String>,
}

impl JustificationPlan {
    pub fn new(line_cluster_range: IntRange, allocations: Vec<JustificationAllocation>, deficit_before: f64, unfilled_deficit: f64, fallback_reason: Option<String>) -> Self {
        Self {
            line_cluster_range,
            allocations,
            deficit_before,
            unfilled_deficit,
            fallback_reason: match fallback_reason { Some(v) => Some(v.to_string()), None => None },
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "JustificationPlan(",
            "lineClusterRange=",
            (self.line_cluster_range).clone().to_string(),
            ", ",
            "allocations=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.allocations).clone();
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
            "deficitBefore=",
            self.deficit_before,
            ", ",
            "unfilledDeficit=",
            self.unfilled_deficit,
            ", ",
            "fallbackReason=",
            match (self.fallback_reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompressionPlan {
    pub allocations: Vec<PushInAllocation>,
    pub surplus_before: f64,
    pub unfilled_surplus: f64,
}

impl CompressionPlan {
    pub fn new(allocations: Vec<PushInAllocation>, surplus_before: f64, unfilled_surplus: f64) -> Self {
        Self {
            allocations,
            surplus_before,
            unfilled_surplus,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "CompressionPlan(",
            "allocations=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.allocations).clone();
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
            "surplusBefore=",
            self.surplus_before,
            ", ",
            "unfilledSurplus=",
            self.unfilled_surplus,
            ")"
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct Justifier {
    pub word_space_max_em: f64,
    pub progressive_technical_whitespace_stretch_max_em: f64,
}

impl Justifier {
    pub fn new(word_space_max_em: Option<f64>, progressive_technical_whitespace_stretch_max_em: Option<f64>) -> Self {
        let word_space_max_em = word_space_max_em.unwrap_or_else(|| 0.5);
        let progressive_technical_whitespace_stretch_max_em = progressive_technical_whitespace_stretch_max_em.unwrap_or_else(|| 0.25);
        Self {
            word_space_max_em: word_space_max_em,
            progressive_technical_whitespace_stretch_max_em: progressive_technical_whitespace_stretch_max_em,
        }
    }

    pub fn progressive_technical_whitespace_stretch_capacity(&self, font_size: f64) -> f64 {
        return self.progressive_technical_whitespace_stretch_max_em * font_size;
    }

    pub fn justify(&self, c: &Vec<Cluster>, roles: &Vec<FontRole>, edges: &Vec<EastAsianSpacingEdges>, r: IntRange, max_width: f64, font_size: f64, skip: bool, skip_reason: Option<String>, allow_sino_western_gap_stretch: Option<bool>, base_em: f64, max_em: f64, no_stretch:
Option<SortedSetTable<u32>>, no_stretch_after: Option<SortedSetTable<u32>>, bracket: Option<SortedSetTable<u32>>, physical: Option<SortedSetTable<u32>>, r#virtual: Option<SortedMapTable<u32, u32>>, virtual_sino: Option<SortedSetTable<u32>>, uniform_object:
Option<SortedSetTable<u32>>, preferred: Option<SortedMapTable<u32, InlineObjectPreferredStretch>>, technical: Option<SortedMapTable<u32, ProgressiveBreakTier>>, emergency: Option<SortedMapTable<u32, String>>, preferred_emergency: Option<SortedMapTable<u32, String>>) ->
Result<JustificationPlan, TextRangeError> {
        let ns = match &(no_stretch) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option) =>
(*__option).clone() };
        let nsa = match &(no_stretch_after) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option1) =>
(*__option1).clone() };
        let br = match &(bracket) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option2) =>
(*__option2).clone() };
        let ph = match &(physical) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option3) =>
(*__option3).clone() };
        let vi = match &(r#virtual) { None => SortedTable::sorted_table_map_builder::<u32, u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option4) =>
(*__option4).clone() };
        let vs = match &(virtual_sino) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option5) =>
(*__option5).clone() };
        let uo = match &(uniform_object) { None => SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option6) =>
(*__option6).clone() };
        let pref = match &(preferred) { None => SortedTable::sorted_table_map_builder::<u32, InlineObjectPreferredStretch>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
Some(__option7) => (*__option7).clone() };
        let te = match &(technical) { None => SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakTier>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(),
Some(__option8) => (*__option8).clone() };
        let emg = match &(emergency) { None => SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option9) =>
(*__option9).clone() };
        let pem = match &(preferred_emergency) { None => SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), Some(__option10)
=> (*__option10).clone() };
        if u32::try_from((roles.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "clusterRoles must align with adjustedClusters.".to_string() });
        }
        if u32::try_from((edges.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "East_Asian_Spacing values must align with adjustedClusters.".to_string() });
        }
        let mut width_terms: Vec<f64> = vec![];
        let mut i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
            width_terms.push(c[usize::try_from(i).unwrap_or(0)].advance);
            i = u32::wrapping_add(i, 1);
        }
        let width = AccurateSum::accurate_sum_of(&width_terms);
        let deficit = { let __min_a = (max_width - width) as f64; let __min_b = 0.0f64 as f64; if __min_a.is_nan() || __min_b.is_nan() { f64::NAN } else { if __min_a > __min_b { __min_a } else if __min_b > __min_a { __min_b } else if __min_a == 0.0 && __min_b == 0.0 { if
__min_a.is_sign_negative() { __min_b } else { __min_a } } else { __min_a } } };
        if skip || (deficit) <= 0 as f64 {
            return Ok(JustificationPlan::new((r).clone(), vec![].to_vec(), deficit, deficit, if skip { skip_reason } else { None }.clone()));
        }
        let remaining: Arc<Mutex<f64>> = Arc::new(Mutex::new(deficit));
        let out: Arc<Mutex<Vec<JustificationAllocation>>> = Arc::new(Mutex::new(Vec::new()));
        let boundary_is_closed: Arc<dyn Fn(u32, u32) -> bool + Send + Sync + 'static> = { let nsa = (nsa).clone(); let ns = (ns).clone(); Arc::new(move |l, x| {
        return nsa.has(&(l)) || ns.has(&(l)) || ns.has(&(x));
}) };
        let space_gap_is_closed: Arc<dyn Fn(u32) -> bool + Send + Sync + 'static> = { let nsa = (nsa).clone(); let ns = (ns).clone(); Arc::new(move |x| {
        return nsa.has(&(x - 1)) || nsa.has(&(x)) || ns.has(&(x - 1)) || ns.has(&(x + 1));
}) };
        let alloc: Arc<dyn Fn(&Vec<JustificationOpportunity>, &str) -> () + Send + Sync + '_> = { let remaining = (remaining).clone(); let out = Arc::clone(&out); Arc::new({ let remaining = Arc::clone(&remaining); let out = Arc::clone(&out); move |ops, reason| {
        if u32::try_from((ops.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || (*remaining.lock().unwrap()) <= 0 as f64 {
            return;
        }
        let mut capacity_terms: Vec<f64> = vec![];
        {
            let mut _g = 0u32;
            while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ops.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                let o = (ops[usize::try_from(_g).unwrap_or(0)]).clone();
                _g = u32::wrapping_add(_g, 1);
                capacity_terms.push(o.capacity);
            }
        }
        let total = AccurateSum::accurate_sum_of(&capacity_terms);
        if total <= 0 as f64 {
            return;
        }
        if total >= *remaining.lock().unwrap() {
            let f = *remaining.lock().unwrap() / total;
            Functional::for_each(&ops, { let out = Arc::clone(&out); move |o: &JustificationOpportunity| {
        let d = o.capacity * f;
        if d > (0 as f64) {
            out.lock().unwrap().push(JustificationAllocation::new(o.target_cluster_index, o.kind, o.priority, d, match &(o.reason) { None => reason.to_string(), Some(__option12) => (*__option12).clone() }.as_str()));
        }
} });
            *remaining.lock().unwrap() = 0 as f64 as f64;
        } else {
            Functional::for_each(&ops, { let out = Arc::clone(&out); move |o: &JustificationOpportunity| {
        if o.capacity > (0 as f64) {
            out.lock().unwrap().push(JustificationAllocation::new(o.target_cluster_index, o.kind, o.priority, o.capacity, match &(o.reason) { None => reason.to_string(), Some(__option14) => (*__option14).clone() }.as_str()));
        }
} });
            *remaining.lock().unwrap() -= total;
        }
} }) };
        let build: Arc<dyn Fn(GlueKind, u32, f64, Option<String>, Arc<dyn Fn(u32, u32) -> bool + Send + Sync>) -> Vec<JustificationOpportunity> + Send + Sync + 'static> = { let r = (r).clone(); Arc::new(move |kind, priority, cap, reason, p| {
        let mut a: Vec<JustificationOpportunity> = vec![];
        if cap <= 0 as f64 {
            return a;
        }
        let mut j = r.start;
        while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((r.end).to_ne_bytes())) {
            if p(j, u32::wrapping_add(j, 1)) {
                a.push(JustificationOpportunity::new(j, kind, priority, cap, reason.clone()));
            }
            j = u32::wrapping_add(j, 1);
        }
        return a;
}) };
        let finish: Arc<dyn Fn(Option<String>) -> JustificationPlan + Send + Sync + 'static> = { let r = (r).clone(); let out = Arc::clone(&out); let remaining = (remaining).clone(); Arc::new({ let out = Arc::clone(&out); let remaining = Arc::clone(&remaining); move |fallback| {
        return JustificationPlan::new((r).clone(), out.lock().unwrap().to_vec(), deficit, { let __min_a1 = (*remaining.lock().unwrap()) as f64; let __min_b1 = 0.0f64 as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 > __min_b1 { __min_a1 } else if
__min_b1 > __min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_b1 } else { __min_a1 } } else { __min_a1 } } }, fallback.clone());
} }) };
        let mut ops = build(GlueKind::ProgressiveTechnical, ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Whitespace), self.progressive_technical_whitespace_stretch_capacity(font_size),
Some("ProgressiveTechnicalWhitespaceStretch".to_string()), { let te = (te).clone(); let c = (c).to_vec(); Arc::new(move |l, _x| {
        let t = te.get(&(l));
        return t == Some(ProgressiveBreakTier::Whitespace) && JustifierFields::justifier_fields_all_whitespace(((c[usize::try_from(l).unwrap_or(0)]).clone().text).to_string().as_str());
}) });
        alloc(&ops, &"ProgressiveTechnicalWhitespaceStretch");
        if *remaining.lock().unwrap() <= 0 as f64 {
            return Ok(finish(None.clone()));
        }
        let mut ws: Vec<JustificationOpportunity> = vec![];
        i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
            if JustifierFields::justifier_fields_word_space_between_narrow(&c, i, &edges) && !space_gap_is_closed(i) {
                let h = { let __min_a2 = (self.word_space_max_em * font_size - c[usize::try_from(i).unwrap_or(0)].advance) as f64; let __min_b2 = 0.0f64 as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 >
__min_a2 { __min_b2 } else if __min_a2 == 0.0 && __min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } };
                if c[usize::try_from(i).unwrap_or(0)].advance > (0 as f64) && (h) > (0 as f64) {
                    ws.push(JustificationOpportunity::new(i, GlueKind::WordSpace, 0u32, h, None));
                }
            }
            i = u32::wrapping_add(i, 1);
        }
        alloc(&ws, &"WordSpace");
        if *remaining.lock().unwrap() <= 0 as f64 {
            return Ok(finish(None.clone()));
        }
        let mut sino: Vec<JustificationOpportunity> = vec![];
        if allow_sino_western_gap_stretch.unwrap_or(false) {
            sino = build(GlueKind::CjkLatinSpace, 1, { let __min_a4 = ((max_em - base_em) * font_size) as f64; let __min_b4 = 0.0f64 as f64; if __min_a4.is_nan() || __min_b4.is_nan() { f64::NAN } else { if __min_a4 > __min_b4 { __min_a4 } else if __min_b4 > __min_a4 { __min_b4 }
else if __min_a4 == 0.0 && __min_b4 == 0.0 { if __min_a4.is_sign_negative() { __min_b4 } else { __min_a4 } } else { __min_a4 } } }, None.clone(), { let edges = (edges).to_vec(); let ph = (ph).clone(); let boundary_is_closed = (boundary_is_closed).clone(); let c = (c).to_vec();
Arc::new(move |l, x| {
        return JustifierFields::justifier_fields_wide_narrow_boundary(l, x, &edges) && !ph.has(&(l)) && !boundary_is_closed(l, x) && (u32::from_ne_bytes((u_string::find_from(&((c[usize::try_from(l).unwrap_or(0)]).clone().text).to_string(), " ", 0)).to_ne_bytes())) > 2147483647 &&
u32::from_ne_bytes((u_string::find_from(&((c[usize::try_from(x).unwrap_or(0)]).clone().text).to_string(), " ", 0)).to_ne_bytes()) != 0;
}) });
            i = r.start;
            while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
                if vs.has(&(i)) {
                    let prev = vi.get(&(i));
                    if match &(prev) { Some(__option15) => i32::from_ne_bytes((i).to_ne_bytes()) >= i32::from_ne_bytes((r.start).to_ne_bytes()) && (i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) && !ns.has(&(*__option15))
&& !ns.has(&(u32::wrapping_add(i, 1))), None => false } {
                        sino.push(JustificationOpportunity::new(i, GlueKind::CjkLatinSpace, 1u32, { let __min_a6 = ((max_em - base_em) * font_size) as f64; let __min_b6 = 0.0f64 as f64; if __min_a6.is_nan() || __min_b6.is_nan() { f64::NAN } else { if __min_a6 > __min_b6 {
__min_a6 } else if __min_b6 > __min_a6 { __min_b6 } else if __min_a6 == 0.0 && __min_b6 == 0.0 { if __min_a6.is_sign_negative() { __min_b6 } else { __min_a6 } } else { __min_a6 } } }, Some("AttachedInlineVirtualAutoSpace".to_string())));
                    }
                }
                if JustifierFields::justifier_fields_wide_narrow_typed_space(&c, i, &edges) && !space_gap_is_closed(i) && (c[usize::try_from(i).unwrap_or(0)].advance) > (0 as f64) {
                    let h = { let __min_a7 = (max_em * font_size - c[usize::try_from(i).unwrap_or(0)].advance) as f64; let __min_b7 = 0.0f64 as f64; if __min_a7.is_nan() || __min_b7.is_nan() { f64::NAN } else { if __min_a7 > __min_b7 { __min_a7 } else if __min_b7 > __min_a7 {
__min_b7 } else if __min_a7 == 0.0 && __min_b7 == 0.0 { if __min_a7.is_sign_negative() { __min_b7 } else { __min_a7 } } else { __min_a7 } } };
                    if h > (0 as f64) {
                        sino.push(JustificationOpportunity::new(i, GlueKind::CjkLatinSpace, 1u32, h, None));
                    }
                }
                i = u32::wrapping_add(i, 1);
            }
        }
        alloc(&sino, &"CjkLatinSpace");
        if *remaining.lock().unwrap() <= 0 as f64 {
            return Ok(finish(None.clone()));
        }
        let kinds = vec![
    InlineObjectPreferredStretchKind::PunctuationTrailing,
    InlineObjectPreferredStretchKind::Relation,
    InlineObjectPreferredStretchKind::BinaryOperator,
];
        let mut ki = 0u32;
        while (i32::from_ne_bytes((ki).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((kinds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let mut po: Vec<JustificationOpportunity> = vec![];
            let mut mi = 0u32;
            while (i32::from_ne_bytes((mi).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((pref.size()).to_ne_bytes())).to_ne_bytes())) {
                let key = pref.key_at(i32::from_ne_bytes((mi).to_ne_bytes()));
                let pv = pref.get(&(key));
                if match &(pv) { Some(__option16) => __option16.kind == kinds[usize::try_from(ki).unwrap_or(0)] && (i32::from_ne_bytes((key).to_ne_bytes())) >= i32::from_ne_bytes((r.start).to_ne_bytes()) && (i32::from_ne_bytes((key).to_ne_bytes())) <
(i32::from_ne_bytes((r.end).to_ne_bytes())) && !boundary_is_closed(key, u32::wrapping_add(key, 1)), None => false } {
                    po.push(JustificationOpportunity::new(key, JustifierFields::justifier_fields_glue_kind(kinds[usize::try_from(ki).unwrap_or(0)]), 2u32, pv.as_ref().unwrap().get_capacity(),
Some((JustifierFields::justifier_fields_reason(kinds[usize::try_from(ki).unwrap_or(0)])).to_string())));
                }
                mi = u32::wrapping_add(mi, 1);
            }
            alloc(&po, JustifierFields::justifier_fields_reason(kinds[usize::try_from(ki).unwrap_or(0)]).as_str());
            if *remaining.lock().unwrap() <= 0 as f64 {
                return Ok(finish(None.clone()));
            }
            ki = u32::wrapping_add(ki, 1);
        }
        ops = build(GlueKind::EmergencyGraphemeTracking, 3, *remaining.lock().unwrap(), None.clone(), { let pem = (pem).clone(); Arc::new(move |l, _x| {
        return pem.has(&(l));
}) });
        let capacity = ops.len();
        let mut pipeline_result = Vec::with_capacity(capacity);
        for pipeline_index in 0..match u32::try_from(ops.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (ops[usize::try_from(pipeline_index).unwrap_or(0)]).clone();
            pipeline_result.push(JustificationOpportunity::new(v.target_cluster_index, v.kind, v.priority, v.capacity, Some((format!("{}{}",
            "TerminalTechnicalEmergencyTracking:",
            match pem.get(&(v.target_cluster_index)) { Some(ref v) => v.to_string(), None => "null".to_string() }
        )).to_string())));
        }
        let pe = (pipeline_result).clone();
        alloc(&pe, &"TerminalTechnicalEmergencyTracking");
        if *remaining.lock().unwrap() <= 0 as f64 {
            return Ok(finish(None.clone()));
        }
        let mut has_cjk = false;
        i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
            if edges[usize::try_from(i).unwrap_or(0)].contains_wide {
                has_cjk = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut has_u = false;
        i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((r.end).to_ne_bytes())) {
            if uo.has(&(i)) && !boundary_is_closed(i, u32::wrapping_add(i, 1)) {
                has_u = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut has_e = false;
        i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((r.end).to_ne_bytes())) {
            if emg.has(&(i)) {
                has_e = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        if !has_cjk && !has_u && !has_e {
            return Ok(finish(Some("WesternDominantLineNaturalSpacing".to_string())));
        }
        let mut all: Vec<JustificationOpportunity> = vec![];
        let text_ops = build(GlueKind::CjkInterChar, 3, *remaining.lock().unwrap(), None.clone(), { let roles = (roles).to_vec(); let edges = (edges).to_vec(); let br = (br).clone(); let ph = (ph).clone(); let vi = (vi).clone(); let uo = (uo).clone(); let boundary_is_closed =
(boundary_is_closed).clone(); Arc::new(move |l, x| {
        let both = JustifierFields::justifier_fields_is_cjk(roles[usize::try_from(l).unwrap_or(0)]) && JustifierFields::justifier_fields_is_cjk(roles[usize::try_from(x).unwrap_or(0)]);
        let pw = roles[usize::try_from(l).unwrap_or(0)] == FontRole::CjkPunctuation && edges[usize::try_from(x).unwrap_or(0)].leading == EastAsianSpacingValue::Narrow || edges[usize::try_from(l).unwrap_or(0)].trailing == EastAsianSpacingValue::Narrow &&
roles[usize::try_from(x).unwrap_or(0)] == FontRole::CjkPunctuation;
        let vw = allow_sino_western_gap_stretch.unwrap_or(false) && JustifierFields::justifier_fields_wide_narrow_boundary(l, x, &edges);
        return (both || pw || vw) && !br.has(&(l)) && !ph.has(&(l)) && !vi.has(&(l)) && !uo.has(&(l)) && !boundary_is_closed(l, x);
}) });
        all = { let mut result = all.clone(); result.extend(text_ops.iter().cloned()); result };
        all = { let mut result = all.clone(); result.extend(build(GlueKind::CjkInterChar, 3, *remaining.lock().unwrap(), Some("WesternBracketCjkInterChar".to_string()), { let br = (br).clone(); let ph = (ph).clone(); let uo = (uo).clone(); let boundary_is_closed =
(boundary_is_closed).clone(); Arc::new(move |l, x| {
        return br.has(&(l)) && !ph.has(&(l)) && !uo.has(&(l)) && !boundary_is_closed(l, x);
}) }).iter().cloned()); result };
        all = { let mut result = all.clone(); result.extend(build(GlueKind::CjkInterChar, 3, *remaining.lock().unwrap(), Some("AttachedInlineVirtualInterChar".to_string()), { let vi = (vi).clone(); let vs = (vs).clone(); let uo = (uo).clone(); let ns = (ns).clone(); let nsa =
(nsa).clone(); Arc::new(move |l, x| {
        let pv = vi.get(&(l));
        return match &(pv) { Some(__option20) => (allow_sino_western_gap_stretch.unwrap_or(false) || !vs.has(&(l))) && !uo.has(&(l)) && !ns.has(&(*__option20)) && !ns.has(&(x)) && !nsa.has(&(*__option20)), None => false };
}) }).iter().cloned()); result };
        all = { let mut result = all.clone(); result.extend(build(GlueKind::InlineObjectBoundary, 3, *remaining.lock().unwrap(), None.clone(), { let uo = (uo).clone(); let boundary_is_closed = (boundary_is_closed).clone(); Arc::new(move |l, x| {
        return uo.has(&(l)) && !boundary_is_closed(l, x);
}) }).iter().cloned()); result };
        i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
            if (JustifierFields::justifier_fields_word_space_between_narrow(&c, i, &edges) || allow_sino_western_gap_stretch.unwrap_or(false) && JustifierFields::justifier_fields_wide_narrow_typed_space(&c, i, &edges)) && !space_gap_is_closed(i) &&
(c[usize::try_from(i).unwrap_or(0)].advance) > (0 as f64) {
                all.push(JustificationOpportunity::new(i, GlueKind::CjkInterChar, 3u32, *remaining.lock().unwrap(), None));
            }
            i = u32::wrapping_add(i, 1);
        }
        alloc(&all, &"CjkInterChar");
        if *remaining.lock().unwrap() <= 0 as f64 {
            return Ok(finish(None.clone()));
        }
        ops = build(GlueKind::EmergencyGraphemeTracking, 4, *remaining.lock().unwrap(), None.clone(), { let emg = (emg).clone(); let pem = (pem).clone(); Arc::new(move |l, _x| {
        return emg.has(&(l)) && !pem.has(&(l));
}) });
        let capacity = ops.len();
        let mut pipeline_result1 = Vec::with_capacity(capacity);
        for pipeline_index1 in 0..match u32::try_from(ops.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (ops[usize::try_from(pipeline_index1).unwrap_or(0)]).clone();
            pipeline_result1.push(JustificationOpportunity::new(v.target_cluster_index, v.kind, v.priority, v.capacity, Some((format!("{}{}",
            "EmergencyGraphemeTracking:",
            match emg.get(&(v.target_cluster_index)) { Some(ref v) => v.to_string(), None => "null".to_string() }
        )).to_string())));
        }
        let ee = (pipeline_result1).clone();
        alloc(&ee, &"EmergencyGraphemeTracking");
        return Ok(finish(if *remaining.lock().unwrap() > (0 as f64) && has_e { Some("EmergencyTrackingNoOpenBoundary".to_string()) } else { None }.clone()));
    }

    pub fn compress(&self, surplus: f64, opps: &Vec<ShrinkOpportunity>) -> Result<CompressionPlan, TextRangeError> {
        if surplus <= 0 as f64 {
            return Ok(CompressionPlan::new(vec![].to_vec(), 0 as f64 as f64, 0 as f64 as f64));
        }
        let mut rem = surplus;
        let mut out: Vec<PushInAllocation> = vec![];
        let mut pipeline_result: Vec<ShrinkOpportunity> = Vec::new();
        for v in opps {
            if v.capacity > (0 as f64) {
                pipeline_result.push(v.clone());
            }
        }
        let mut pipeline_builder: SortedMapTableBuilder<u32, Vec<ShrinkOpportunity>> = SortedTable::sorted_table_map_builder::<u32, Vec<ShrinkOpportunity>>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for o in &pipeline_result {
            let pipeline_entry = ShrinkTierEntry { key: o.tier, value: (*o).clone() };
            let mut pipeline_bucket = match pipeline_builder.get(&(pipeline_entry.key)) {
                Some(b) => b,
                None => Vec::new(),
            };
            pipeline_bucket.push((pipeline_entry.value).clone());
            pipeline_builder.put(&(pipeline_entry.key), &pipeline_bucket);
        }
        let pipeline_result1: SortedMapTable<u32, Vec<ShrinkOpportunity>> = pipeline_builder.clone().build();
        let m: SortedMapTable<u32, Vec<ShrinkOpportunity>> = (pipeline_result1).clone();
        let mut k = 0u32;
        while (i32::from_ne_bytes((k).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((m.size()).to_ne_bytes())).to_ne_bytes())) {
            if rem <= 0 as f64 {
                break;
            }
            let group = m.value_at(i32::from_ne_bytes((k).to_ne_bytes()));
            let mut group_terms: Vec<f64> = vec![];
            for gi in 0..match u32::try_from(group.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                group_terms.push(group[usize::try_from(gi).unwrap_or(0)].capacity);
            }
            let total = AccurateSum::accurate_sum_of(&group_terms);
            if total <= 0 as f64 {
                k = u32::wrapping_add(k, 1);
                continue;
            }
            let f = { let __min_a8 = 1.0f64 as f64; let __min_b8 = (rem / total) as f64; if __min_a8.is_nan() || __min_b8.is_nan() { f64::NAN } else { if __min_a8 < __min_b8 { __min_a8 } else if __min_b8 < __min_a8 { __min_b8 } else if __min_a8 == 0.0 && __min_b8 == 0.0 { if
__min_a8.is_sign_negative() { __min_a8 } else { __min_b8 } } else { __min_a8 } } };
            for q in &group {
                let d = q.capacity * f;
                if d > (0 as f64) {
                    out.push(PushInAllocation::new(q.cluster_index, d, q.capacity, Some(q.channel))?);
                }
            }
            rem -= total * f;
            k = u32::wrapping_add(k, 1);
        }
        return Ok(CompressionPlan::new(out.to_vec(), surplus, { let __min_a9 = rem as f64; let __min_b9 = 0.0f64 as f64; if __min_a9.is_nan() || __min_b9.is_nan() { f64::NAN } else { if __min_a9 > __min_b9 { __min_a9 } else if __min_b9 > __min_a9 { __min_b9 } else if __min_a9 ==
0.0 && __min_b9 == 0.0 { if __min_a9.is_sign_negative() { __min_b9 } else { __min_a9 } } else { __min_a9 } } }));
    }
}

#[derive(Clone, Copy)]
pub struct JustifierFields;

impl JustifierFields {
    pub fn justifier_fields_all_whitespace(s: &str) -> bool {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        if __count == 0 {
            return false;
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            if !string_tools::StringTools::string_tools_is_space(s, i32::from_ne_bytes((i).to_ne_bytes())) {
                return false;
            }
            i = u32::wrapping_add(i, 1);
        }
        return true;
    }

    pub fn justifier_fields_is_cjk(r: FontRole) -> bool {
        return r == FontRole::CjkText || r == FontRole::CjkPunctuation;
    }

    pub fn justifier_fields_glue_kind(k: InlineObjectPreferredStretchKind) -> GlueKind {
        return match k {
            InlineObjectPreferredStretchKind::PunctuationTrailing => GlueKind::InlineObjectPunctuationTrailing,
            InlineObjectPreferredStretchKind::Relation => GlueKind::InlineObjectRelation,
            InlineObjectPreferredStretchKind::BinaryOperator => GlueKind::InlineObjectBinaryOperator,
        };
    }

    pub fn justifier_fields_reason(k: InlineObjectPreferredStretchKind) -> String {
        return match k {
            InlineObjectPreferredStretchKind::PunctuationTrailing => "InlineObjectPunctuationTrailing".to_string().to_string(),
            InlineObjectPreferredStretchKind::Relation => "InlineObjectRelation".to_string().to_string(),
            InlineObjectPreferredStretchKind::BinaryOperator => "InlineObjectBinaryOperator".to_string().to_string(),
        };
    }

    pub fn justifier_fields_wide_narrow_boundary(l: u32, x: u32, e: &Vec<EastAsianSpacingEdges>) -> bool {
        return JustifierFields::justifier_fields_wide_narrow_pair_with(Some(e[usize::try_from(l).unwrap_or(0)].trailing), Some(e[usize::try_from(x).unwrap_or(0)].leading));
    }

    pub fn justifier_fields_wide_narrow_pair_with(a: Option<EastAsianSpacingValue>, b: Option<EastAsianSpacingValue>) -> bool {
        return a == Some(EastAsianSpacingValue::Wide) && b == Some(EastAsianSpacingValue::Narrow) || a == Some(EastAsianSpacingValue::Narrow) && b == Some(EastAsianSpacingValue::Wide);
    }

    pub fn justifier_fields_word_space_between_narrow(c: &Vec<Cluster>, i: u32, e: &Vec<EastAsianSpacingEdges>) -> bool {
        if i > 2147483647 || (i32::from_ne_bytes((i).to_ne_bytes())) >= i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) ||
!JustifierFields::justifier_fields_all_spaces(((c[usize::try_from(i).unwrap_or(0)]).clone().text).to_string().as_str()) {
            return false;
        }
        return (i32::from_ne_bytes((i).to_ne_bytes())) > (0) && (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::wrapping_sub(u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).to_ne_bytes())) && e[usize::try_from(u32::wrapping_sub(i,
1)).unwrap_or(0)].trailing == EastAsianSpacingValue::Narrow && e[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)].leading == EastAsianSpacingValue::Narrow && !JustifierFields::justifier_fields_all_spaces(((c[usize::try_from(u32::wrapping_sub(i,
1)).unwrap_or(0)]).clone().text).to_string().as_str()) && !JustifierFields::justifier_fields_all_spaces(((c[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]).clone().text).to_string().as_str());
    }

    pub fn justifier_fields_wide_narrow_typed_space(c: &Vec<Cluster>, i: u32, e: &Vec<EastAsianSpacingEdges>) -> bool {
        if i > 2147483647 || (i32::from_ne_bytes((i).to_ne_bytes())) >= i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) ||
!JustifierFields::justifier_fields_all_spaces(((c[usize::try_from(i).unwrap_or(0)]).clone().text).to_string().as_str()) {
            return false;
        }
        let l = if i32::from_ne_bytes((i).to_ne_bytes()) > (0) { Some(e[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)].trailing) } else { None };
        let x = if i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some(e[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)].leading) } else { None };
        return JustifierFields::justifier_fields_wide_narrow_pair_with(l, x);
    }

    pub fn justifier_fields_all_spaces(s: &str) -> bool {
    let __units1 = u_string::units(&s);
    let __count1 = u_string::unit_count(&s);
        if __count1 == 0 {
            return false;
        }
        let mut i = 0u32;
        let __units2 = u_string::units(&s);
        let __count2 = u_string::unit_count(&s);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count1).to_ne_bytes())) {
            if u_string::char_at_from(&__units2, i) != " " {
                return false;
            }
            i = u32::wrapping_add(i, 1);
        }
        return true;
    }
}
