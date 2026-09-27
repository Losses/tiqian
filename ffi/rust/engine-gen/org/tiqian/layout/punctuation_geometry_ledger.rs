use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_edge_trim_decision_info::LineEdgeTrimDecisionInfo;
use crate::org::tiqian::core::spacing_decision_info::SpacingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::punctuation_model::PunctuationAnchor;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressionResult;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaryResolver;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Clone, PartialEq)]
pub struct PunctuationGeometryLedger {
    pub natural_clusters: Vec<Cluster>,
    pub geometries: SortedMapTable<u32, PunctuationClusterGeometry>,
    pub budgets: SortedMapTable<u32, GlueBudget>,
    pub justification_delta_by_cluster: SortedMapTable<u32, f64>,
    pub raw_edge_trim_by_cluster: SortedMapTable<u32, f64>,
    pub ruby_spread_by_cluster: SortedMapTable<u32, f64>,
    pub inline_box_advance_by_cluster: SortedMapTable<u32, f64>,
    pub attached_inline_trailing_glue_by_cluster: SortedMapTable<u32, f64>,
}

impl PunctuationGeometryLedger {
    pub fn new(natural_clusters: Vec<Cluster>, geometries: SortedMapTable<u32, PunctuationClusterGeometry>, budgets: SortedMapTable<u32, GlueBudget>, justification_delta_by_cluster: Option<SortedMapTable<u32, f64>>, raw_edge_trim_by_cluster: Option<SortedMapTable<u32, f64>>, ruby_spread_by_cluster: Option<SortedMapTable<u32, f64>>, inline_box_advance_by_cluster: Option<SortedMapTable<u32, f64>>, attached_inline_trailing_glue_by_cluster: Option<SortedMapTable<u32, f64>>) -> Self {
        let justification_delta_by_cluster = justification_delta_by_cluster.unwrap_or_else(|| PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f());
        let raw_edge_trim_by_cluster = raw_edge_trim_by_cluster.unwrap_or_else(|| PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f());
        let ruby_spread_by_cluster = ruby_spread_by_cluster.unwrap_or_else(|| PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f());
        let inline_box_advance_by_cluster = inline_box_advance_by_cluster.unwrap_or_else(|| PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f());
        let attached_inline_trailing_glue_by_cluster = attached_inline_trailing_glue_by_cluster.unwrap_or_else(|| PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f());
        Self {
            natural_clusters,
            geometries,
            budgets,
            justification_delta_by_cluster: justification_delta_by_cluster,
            raw_edge_trim_by_cluster: raw_edge_trim_by_cluster,
            ruby_spread_by_cluster: ruby_spread_by_cluster,
            inline_box_advance_by_cluster: inline_box_advance_by_cluster,
            attached_inline_trailing_glue_by_cluster: attached_inline_trailing_glue_by_cluster,
        }
    }

    pub fn resolve_clusters(&self) -> Vec<Cluster> {
        let mut result: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from((self.natural_clusters).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (self.natural_clusters[usize::try_from(i).unwrap_or(0)]).clone();
            let v = self.resolved_advance(i, (c).clone());
            let sh = if self.geometries.clone().has(&(i)) { (self.geometries).clone().get(&(i)).as_ref().unwrap().glyph_inline_shift } else { 0 as f64 };
            result.push(if v == c.advance && sh == 0 as f64 { c } else { Cluster::new((c.range).clone(), (c.text).to_ustring().as_ustr(), (c.font_key).to_ustring().as_ustr(), v, Some((c.display_text).to_ustring()), Some(c.baseline_shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift + sh)) });
        }
        return result;
    }

    fn resolved_advance(&self, i: u32, c: Cluster) -> f64 {
        let raw = if self.raw_edge_trim_by_cluster.clone().has(&(i)) { ((self.raw_edge_trim_by_cluster).clone().get(&(i))).unwrap() } else { 0 as f64 };
        let sp = if self.ruby_spread_by_cluster.clone().has(&(i)) { ((self.ruby_spread_by_cluster).clone().get(&(i))).unwrap() } else { 0 as f64 };
        let d = if self.justification_delta_by_cluster.clone().has(&(i)) { ((self.justification_delta_by_cluster).clone().get(&(i))).unwrap() } else { 0 as f64 };
        let att = if self.attached_inline_trailing_glue_by_cluster.clone().has(&(i)) { ((self.attached_inline_trailing_glue_by_cluster).clone().get(&(i))).unwrap() } else { 0 as f64 };
        let r#box = if self.inline_box_advance_by_cluster.clone().has(&(i)) { ((self.inline_box_advance_by_cluster).clone().get(&(i))).unwrap() } else { 0 as f64 };
        if !(self.geometries).clone().has(&(i)) {
            return { let __min_a = 0.0f64 as f64; let __min_b = (c.advance + d + sp + att - raw) as f64; if __min_a.is_nan() || __min_b.is_nan() { f64::NAN } else { if __min_a > __min_b { __min_a } else if __min_b > __min_a { __min_b } else if __min_a == 0.0 && __min_b == 0.0 { if __min_a.is_sign_negative() { __min_b } else { __min_a } } else { __min_a } } };
        }
        let g = (self.geometries).clone().get(&(i));
        if !(self.budgets).clone().has(&(i)) {
            return { let __min_a1 = 0.0f64 as f64; let __min_b1 = (g.as_ref().unwrap().body_width + r#box + d + sp - raw) as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 > __min_b1 { __min_a1 } else if __min_b1 > __min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_b1 } else { __min_a1 } } else { __min_a1 } } };
        }
        let b = (self.budgets).clone().get(&(i));
        return { let __min_a2 = 0.0f64 as f64; let __min_b2 = (g.as_ref().unwrap().body_width + r#box + b.as_ref().unwrap().get_leading_remaining() + b.as_ref().unwrap().get_trailing_remaining() + d + sp + att - raw) as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 == 0.0 && __min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } };
    }

    pub fn with_inline_box_advances(&self, m: SortedMapTable<u32, f64>) -> PunctuationGeometryLedger {
        return if u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes()) == 0 { (self).clone() } else { PunctuationGeometryLedger::punctuation_geometry_ledger_replace((self).clone(), None, None, None, Some((m).clone()), None, None) };
    }

    pub fn with_ruby_spread(&self, m: SortedMapTable<u32, f64>) -> PunctuationGeometryLedger {
        return if u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes()) == 0 { (self).clone() } else { PunctuationGeometryLedger::punctuation_geometry_ledger_replace((self).clone(), None, None, Some((m).clone()), None, None, None) };
    }

    pub fn with_raw_edge_trims(&self, m: SortedMapTable<u32, f64>) -> PunctuationGeometryLedger {
        if u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes()) == 0 {
            return ((self).clone()).clone();
        }
        let mut n: SortedMapTable<u32, f64> = PunctuationGeometryLedger::punctuation_geometry_ledger_clone_f((self.raw_edge_trim_by_cluster).clone());
        for i in 0..u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes()) {
            let k = m.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let mut b: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
            for j in 0..u32::from_ne_bytes(((n.size()) as u32).to_ne_bytes()) {
                b.put(&(n.key_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) })), &(n.value_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) })));
            }
            b.put(&(k), &((if n.has(&(k)) { (n.get(&(k))).unwrap() } else { 0 as f64 }) + m.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
            n = b.clone().build();
        }
        return PunctuationGeometryLedger::punctuation_geometry_ledger_replace((self).clone(), None, Some((n).clone()), None, None, None, None);
    }

    pub fn add_justification_deltas(&self, m: SortedMapTable<u32, f64>) -> PunctuationGeometryLedger {
        return PunctuationGeometryLedger::punctuation_geometry_ledger_replace((self).clone(), Some((m).clone()), None, None, None, None, None);
    }

    fn consume_side(&self, m: SortedMapTable<u32, f64>, lead: bool) -> PunctuationGeometryLedger {
        let mut n: SortedMapTable<u32, GlueBudget> = PunctuationGeometryLedger::punctuation_geometry_ledger_clone_b((self.budgets).clone());
        for i in 0..u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes()) {
            let k = m.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let amount = m.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if n.has(&(k)) && (amount) > (0 as f64) {
                let q = n.get(&(k));
                let replacement = if lead { GlueBudget::new(q.as_ref().unwrap().leading_natural, { let __min_a3 = q.as_ref().unwrap().leading_natural as f64; let __min_b3 = (q.as_ref().unwrap().leading_consumed + amount) as f64; if __min_a3.is_nan() || __min_b3.is_nan() { f64::NAN } else { if __min_a3 < __min_b3 { __min_a3 } else if __min_b3 < __min_a3 { __min_b3 } else if __min_a3 == 0.0 && __min_b3 == 0.0 { if __min_a3.is_sign_negative() { __min_a3 } else { __min_b3 } } else { __min_a3 } } }, q.as_ref().unwrap().trailing_natural, q.as_ref().unwrap().trailing_consumed) } else { GlueBudget::new(q.as_ref().unwrap().leading_natural, q.as_ref().unwrap().leading_consumed, q.as_ref().unwrap().trailing_natural, { let __min_a4 = q.as_ref().unwrap().trailing_natural as f64; let __min_b4 = (q.as_ref().unwrap().trailing_consumed + amount) as f64; if __min_a4.is_nan() || __min_b4.is_nan() { f64::NAN } else { if __min_a4 < __min_b4 { __min_a4 } else if __min_b4 < __min_a4 { __min_b4 } else if __min_a4 == 0.0 && __min_b4 == 0.0 { if __min_a4.is_sign_negative() { __min_a4 } else { __min_b4 } } else { __min_a4 } } }) };
                let mut b: SortedMapTableBuilder<u32, GlueBudget> = SortedTable::sorted_table_map_builder::<u32, GlueBudget>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
                for j in 0..u32::from_ne_bytes(((n.size()) as u32).to_ne_bytes()) {
                    b.put(&(n.key_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) })), &(n.value_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) })));
                }
                b.put(&(k), &(replacement));
                n = b.clone().build();
            }
        }
        return PunctuationGeometryLedger::punctuation_geometry_ledger_replace((self).clone(), None, None, None, None, None, Some((n).clone()));
    }

    pub fn consume_leading_by_cluster(&self, m: SortedMapTable<u32, f64>) -> PunctuationGeometryLedger {
        return self.consume_side((m).clone(), true);
    }

    pub fn consume_trailing_by_cluster(&self, m: SortedMapTable<u32, f64>) -> PunctuationGeometryLedger {
        return self.consume_side((m).clone(), false);
    }

    pub fn glue_capacities(&self) -> SortedMapTable<u32, GlueCapacity> {
        let mut b: SortedMapTableBuilder<u32, GlueCapacity> = SortedTable::sorted_table_map_builder::<u32, GlueCapacity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((((self.budgets).clone().size()) as u32).to_ne_bytes()) {
            let k = (self.budgets).clone().key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let q = (self.budgets).clone().value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if q.get_leading_remaining() > (0 as f64) || (q.get_trailing_remaining()) > (0 as f64) {
                b.put(&(k), &(GlueCapacity::new(q.get_leading_remaining(), q.get_trailing_remaining(), (self.geometries).clone().has(&(k)) && (self.geometries).clone().get(&(k)).as_ref().unwrap().anchor == Some(PunctuationAnchor::Center))));
            }
        }
        return b.clone().build();
    }

    pub fn to_decision_info(&self) -> Vec<ClusterGeometryDecisionInfo> {
        let mut r: Vec<ClusterGeometryDecisionInfo> = vec![];
        for i in 0..u32::from_ne_bytes((((self.geometries).clone().size()) as u32).to_ne_bytes()) {
            let k = (self.geometries).clone().key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let g = (self.geometries).clone().value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let b = (self.budgets).clone().get(&(k));
            r.push(ClusterGeometryDecisionInfo::new((g.range).clone(), (g.source_text).to_ustring().as_ustr(), (g.display_text).to_ustring().as_ustr(), g.base_advance, g.body_width, b.as_ref().unwrap().leading_natural, b.as_ref().unwrap().leading_consumed, b.as_ref().unwrap().trailing_natural, b.as_ref().unwrap().trailing_consumed, if self.justification_delta_by_cluster.clone().has(&(k)) { ((self.justification_delta_by_cluster).clone().get(&(k))).unwrap() } else { 0 as f64 }, self.resolved_advance(k, (self.natural_clusters[usize::try_from(k).unwrap_or(0)]).clone()), &(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,76,101,100,103,101,114])), (g.reason).to_ustring().as_ustr(), Some(if self.ruby_spread_by_cluster.clone().has(&(k)) { ((self.ruby_spread_by_cluster).clone().get(&(k))).unwrap() } else { 0 as f64 }), Some(g.glyph_inline_shift), g.glyph_placement_reason.clone()));
        }
        return r;
    }

    fn consume_spacing(&self, p: PunctuationSpacingCompressionResult) -> PunctuationGeometryLedger {
        let mut n: SortedMapTable<u32, GlueBudget> = PunctuationGeometryLedger::punctuation_geometry_ledger_clone_b((self.budgets).clone());
        {
            let _g1 = p.adjustments.clone();
            for a in &_g1 {
                let mut idx = 4294967295u32;
                for i in 0..match u32::try_from((self.natural_clusters).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if PunctuationGeometryLedger::punctuation_geometry_ledger_is_inside((a.reduction_target_range).clone(), ((self.natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone()) {
                        idx = i;
                        break;
                    }
                }
                if idx <= 2147483647 && n.has(&(idx)) {
                    let q = n.get(&(idx));
                    let replacement: GlueBudget;
                    if self.geometries.clone().get(&(idx)).as_ref().unwrap().anchor == Some(PunctuationAnchor::Center) {
                        let x = { let __min_a6 = (a.reduction / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0)) as f64; let __min_b6 = { let __min_a7 = q.as_ref().unwrap().get_leading_remaining() as f64; let __min_b7 = q.as_ref().unwrap().get_trailing_remaining() as f64; if __min_a7.is_nan() || __min_b7.is_nan() { f64::NAN } else { if __min_a7 < __min_b7 { __min_a7 } else if __min_b7 < __min_a7 { __min_b7 } else if __min_a7 == 0.0 && __min_b7 == 0.0 { if __min_a7.is_sign_negative() { __min_a7 } else { __min_b7 } } else { __min_a7 } } } as f64; if __min_a6.is_nan() || __min_b6.is_nan() { f64::NAN } else { if __min_a6 < __min_b6 { __min_a6 } else if __min_b6 < __min_a6 { __min_b6 } else if __min_a6 == 0.0 && __min_b6 == 0.0 { if __min_a6.is_sign_negative() { __min_a6 } else { __min_b6 } } else { __min_a6 } } };
                        replacement = GlueBudget::new(q.as_ref().unwrap().leading_natural, q.as_ref().unwrap().leading_consumed + x, q.as_ref().unwrap().trailing_natural, q.as_ref().unwrap().trailing_consumed + x);
                    } else {
                        if q.as_ref().unwrap().get_trailing_remaining() >= q.as_ref().unwrap().get_leading_remaining() {
                            replacement = GlueBudget::new(q.as_ref().unwrap().leading_natural, q.as_ref().unwrap().leading_consumed, q.as_ref().unwrap().trailing_natural, { let __min_a8 = q.as_ref().unwrap().trailing_natural as f64; let __min_b8 = (q.as_ref().unwrap().trailing_consumed + a.reduction) as f64; if __min_a8.is_nan() || __min_b8.is_nan() { f64::NAN } else { if __min_a8 < __min_b8 { __min_a8 } else if __min_b8 < __min_a8 { __min_b8 } else if __min_a8 == 0.0 && __min_b8 == 0.0 { if __min_a8.is_sign_negative() { __min_a8 } else { __min_b8 } } else { __min_a8 } } });
                        } else {
                            replacement = GlueBudget::new(q.as_ref().unwrap().leading_natural, { let __min_a9 = q.as_ref().unwrap().leading_natural as f64; let __min_b9 = (q.as_ref().unwrap().leading_consumed + a.reduction) as f64; if __min_a9.is_nan() || __min_b9.is_nan() { f64::NAN } else { if __min_a9 < __min_b9 { __min_a9 } else if __min_b9 < __min_a9 { __min_b9 } else if __min_a9 == 0.0 && __min_b9 == 0.0 { if __min_a9.is_sign_negative() { __min_a9 } else { __min_b9 } } else { __min_a9 } } }, q.as_ref().unwrap().trailing_natural, q.as_ref().unwrap().trailing_consumed);
                        }
                    }
                    let mut b: SortedMapTableBuilder<u32, GlueBudget> = SortedTable::sorted_table_map_builder::<u32,
GlueBudget>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
                    for j in 0..u32::from_ne_bytes(((n.size()) as u32).to_ne_bytes()) {
                        b.put(&(n.key_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) })), &(n.value_at({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) })));
                    }
                    b.put(&(idx), &(replacement));
                    n = b.clone().build();
                }
            }
        }
        return PunctuationGeometryLedger::punctuation_geometry_ledger_replace((self).clone(), None, None, None, None, None, Some((n).clone()));
    }

    pub fn consume_line_edge_glue(&self, lines: &Vec<LineCandidate>, force: Option<bool>) -> LineEdgeTrimResult {
        let _gthis = (self).clone();
        if u32::try_from((lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || u32::from_ne_bytes((((self.budgets).clone().size()) as u32).to_ne_bytes()) == 0 {
            return LineEdgeTrimResult::new((self).clone(), vec![].to_vec());
        }
        let lead: Arc<Mutex<SortedMapTable<u32, f64>>> = Arc::new(Mutex::new(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()));
        let trail: Arc<Mutex<SortedMapTable<u32, f64>>> = Arc::new(Mutex::new(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()));
        let ds: Arc<Mutex<Vec<LineEdgeTrimDecisionInfo>>> = Arc::new(Mutex::new(vec![]));
        let force_end = match &(force) { None => true, Some(__option) => *__option };
        let consume_at_edge: Arc<dyn Fn(LineCandidate, u32, &UStr) -> () + Send + Sync + '_> = { let _gthis = (_gthis).clone(); let lead = (lead).clone(); let trail = (trail).clone(); let ds = (ds).clone(); Arc::new({ let lead = Arc::clone(&lead); let trail = Arc::clone(&trail); let ds = Arc::clone(&ds); move |line, index, edge| {
        if !_gthis.budgets.has(&(index)) {
            return;
        }
        let q = _gthis.budgets.get(&(index));
        let already_lead = (lead.lock().unwrap().get(&(index))).unwrap_or(0.0f64);
        let already_trail = (trail.lock().unwrap().get(&(index))).unwrap_or(0.0f64);
        let lr = { let __min_a10 = 0.0f64 as f64; let __min_b10 = (q.as_ref().unwrap().get_leading_remaining() - already_lead) as f64; if __min_a10.is_nan() || __min_b10.is_nan() { f64::NAN } else { if __min_a10 > __min_b10 { __min_a10 } else if __min_b10 > __min_a10 { __min_b10 } else if __min_a10 == 0.0 && __min_b10 == 0.0 { if __min_a10.is_sign_negative() { __min_b10 } else { __min_a10 } } else { __min_a10 } } };
        let tr = { let __min_a11 = 0.0f64 as f64; let __min_b11 = (q.as_ref().unwrap().get_trailing_remaining() - already_trail) as f64; if __min_a11.is_nan() || __min_b11.is_nan() { f64::NAN } else { if __min_a11 > __min_b11 { __min_a11 } else if __min_b11 > __min_a11 { __min_b11 } else if __min_a11 == 0.0 && __min_b11 == 0.0 { if __min_a11.is_sign_negative() { __min_b11 } else { __min_a11 } } else { __min_a11 } } };
        let paired = _gthis.geometries.has(&(index)) && _gthis.geometries.get(&(index)).as_ref().unwrap().anchor == Some(PunctuationAnchor::Center);
        let lp = if paired { { let __min_a12 = lr as f64; let __min_b12 = tr as f64; if __min_a12.is_nan() || __min_b12.is_nan() { f64::NAN } else { if __min_a12 < __min_b12 { __min_a12 } else if __min_b12 < __min_a12 { __min_b12 } else if __min_a12 == 0.0 && __min_b12 == 0.0 { if __min_a12.is_sign_negative() { __min_a12 } else { __min_b12 } } else { __min_a12 } } } } else { if edge == UString::from("Start") { lr } else { 0 as f64 } };
        let tp = if paired { { let __min_a13 = lr as f64; let __min_b13 = tr as f64; if __min_a13.is_nan() || __min_b13.is_nan() { f64::NAN } else { if __min_a13 < __min_b13 { __min_a13 } else if __min_b13 < __min_a13 { __min_b13 } else if __min_a13 == 0.0 && __min_b13 == 0.0 { if __min_a13.is_sign_negative() { __min_a13 } else { __min_b13 } } else { __min_a13 } } } } else { if edge == UString::from("End") { tr } else { 0 as f64 } };
        let total = lp + tp;
        if total <= 0 as f64 {
            return;
        }
        if lp > (0 as f64) {
            { let __rhs_value = PunctuationGeometryLedger::punctuation_geometry_ledger_put_f((lead.lock().unwrap()).clone(), index, already_lead + lp); *lead.lock().unwrap() = __rhs_value };
        }
        if tp > (0 as f64) {
            { let __rhs_value1 = PunctuationGeometryLedger::punctuation_geometry_ledger_put_f((trail.lock().unwrap()).clone(), index, already_trail + tp); *trail.lock().unwrap() = __rhs_value1 };
        }
        let side: UString;
        let consumed: f64;
        let natural: f64;
        let reason: UString;
        if paired {
            side = UString::from("both").to_ustring();
            consumed = q.as_ref().unwrap().leading_consumed + q.as_ref().unwrap().trailing_consumed;
            natural = q.as_ref().unwrap().leading_natural + q.as_ref().unwrap().trailing_natural;
            reason = if edge == UString::from("Start") { UString::from("LineStartCenteredPunctuationPairedCompression") } else { UString::from("LineEndCenteredPunctuationPairedCompression") };
        } else {
            if edge == UString::from("Start") {
                side = UString::from("leading").to_ustring();
                consumed = q.as_ref().unwrap().leading_consumed;
                natural = q.as_ref().unwrap().leading_natural;
                reason = UString::from("LineStartHalfWidthPunctuation").to_ustring();
            } else {
                side = UString::from("trailing").to_ustring();
                consumed = q.as_ref().unwrap().trailing_consumed;
                natural = q.as_ref().unwrap().trailing_natural;
                reason = UString::from("LineEndHalfWidthPunctuation").to_ustring();
            }
        }
        ds.lock().unwrap().push(LineEdgeTrimDecisionInfo::new((line.source_range).clone(), ((_gthis.natural_clusters[usize::try_from(index).unwrap_or(0)]).clone().range).clone(), side.as_ustr(), total, consumed, natural, reason.as_ustr()));
} }) };
        for line in lines {
            if i32::from_ne_bytes((((line.cluster_range).clone().start) as i32).to_ne_bytes()) > (i32::from_ne_bytes((((line.cluster_range).clone().end) as i32).to_ne_bytes())) {
                continue;
            }
            let end = (line.cluster_range).clone().end;
            if force_end {
                consume_at_edge((line).clone(), end, UStr::new(&[69,110,100]));
            }
            consume_at_edge((line).clone(), (line.cluster_range).clone().start, UStr::new(&[83,116,97,114,116]));
        }
        return LineEdgeTrimResult::new(self.consume_leading_by_cluster((lead.lock().unwrap()).clone()).consume_trailing_by_cluster((trail.lock().unwrap()).clone()), ds.lock().unwrap().to_vec());
    }

    pub fn resolve_attached_inline_punctuation_boundaries(&self, a: &Vec<InlineAttachment>, atoms: &Vec<PunctuationAtom>, em: f64) -> Result<AttachedInlinePunctuationBoundaryResult, TextRangeError> {
        if u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from(((self.natural_clusters).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: UString::from("Inline attachments must align with punctuation geometry clusters.") });
        }
        if u32::from_ne_bytes((((self.budgets).clone().size()) as u32).to_ne_bytes()) == 0 {
            return Ok(AttachedInlinePunctuationBoundaryResult::new((self).clone(), PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f(), vec![].to_vec()));
        }
        let mut has_previous = false;
        for x in a {
            if *x == InlineAttachment::Previous {
                has_previous = true;
            }
        }
        if !has_previous {
            return Ok(AttachedInlinePunctuationBoundaryResult::new((self).clone(), PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f(), vec![].to_vec()));
        }
        let mut updated: SortedMapTable<u32, GlueBudget> = PunctuationGeometryLedger::punctuation_geometry_ledger_clone_b((self.budgets).clone());
        let mut trailing: SortedMapTable<u32, f64> = PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f();
        let mut decisions: Vec<SpacingDecisionInfo> = vec![];
        let boundaries = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&a);
        for boundary in &boundaries {
            let previous = boundary.previous_cluster_index;
            let end = (boundary.attached_cluster_range).clone().end;
            let previous_budget = if updated.has(&(previous)) { updated.get(&(previous)) } else { None };
            let left = match &(previous_budget) { None => 0 as f64, Some(__option1) => __option1.get_trailing_remaining() };
            let mut next = boundary.next_cluster_index;
            if match &(next) { Some(__option2) => ((self.natural_clusters[usize::try_from(*__option2).unwrap_or(0)]).clone().font_key).to_ustring() == UString::from("mandatory-break") && ((self.natural_clusters[usize::try_from(*__option2).unwrap_or(0)]).clone().display_text).to_ustring() == UString::from(""), None => false } {
                next = None;
            }
            let next_budget = match &(next) { Some(__option4) => if updated.has(&(*__option4)) { updated.get(&(*__option4)) } else { None }, None => None };
            let right = match &(next_budget) { None => 0 as f64, Some(__option5) => __option5.get_leading_remaining() };
            let mut left_atom: Option<PunctuationAtom> = None;
            {
                let mut _g = 0u32;
                while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((atoms.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let atom = (atoms[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    if PunctuationGeometryLedger::punctuation_geometry_ledger_is_inside((atom.range).clone(), ((self.natural_clusters[usize::try_from(previous).unwrap_or(0)]).clone().range).clone()) {
                        left_atom = Some(atom.clone());
                    }
                }
            }
            let mut right_atom: Option<PunctuationAtom> = None;
            match &(next) {
                Some(__option6) => {
                    let mut _g = 0u32;
                    while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((atoms.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let atom = (atoms[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        if PunctuationGeometryLedger::punctuation_geometry_ledger_is_inside((atom.range).clone(), ((self.natural_clusters[usize::try_from(*__option6).unwrap_or(0)]).clone().range).clone()) {
                            right_atom = Some(atom.clone());
                            break;
                        }
                    }
                }
                None => {
                }
            }
            let next_char = match &(next) { None => None,
Some(__option7) => if i32::from_ne_bytes(((u_string::unit_count(&(((self.natural_clusters[usize::try_from(*__option7).unwrap_or(0)]).clone().text).to_ustring()))) as i32).to_ne_bytes()) > (0) { Some(u_string::substring(&((self.natural_clusters[usize::try_from(*__option7).unwrap_or(0)]).clone().text).to_ustring(), 0i32, i32::wrapping_add(0i32, 1)).to_ustring()) } else { None } };
            let natural = left + right;
            let adjusted = match &(next) { None => 0 as f64, Some(__option8) => if match &(left_atom) { Some(__option11) => right_atom != None,
None => false } || (match &(left_atom) { Some(__option12) => __option12.punctuation_class == PunctuationClass::Closing && next_char.is_some() && ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((next_char).as_deref().unwrap_or(UStr::new(&[]))), None => false }) { { let __min_a14 = 0.0f64 as f64; let __min_b14 = (natural - em / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0)) as f64; if __min_a14.is_nan() || __min_b14.is_nan() { f64::NAN } else { if __min_a14 > __min_b14 { __min_a14 } else if __min_b14 > __min_a14 { __min_b14 } else if __min_a14 == 0.0 && __min_b14 == 0.0 { if __min_a14.is_sign_negative() { __min_b14 } else { __min_a14 } } else { __min_a14 } } } } else { natural } };
            match &(previous_budget) {
                Some(__option13) => {
                    if left > (0 as f64) {
                    updated = PunctuationGeometryLedger::punctuation_geometry_ledger_put_b((updated).clone(), previous, GlueBudget::new(__option13.leading_natural, __option13.leading_consumed, __option13.trailing_natural, __option13.trailing_natural));
                    }
                }
                None => {
                }
            }
            let kept = { let __min_a15 = right as f64; let __min_b15 = adjusted as f64; if __min_a15.is_nan() || __min_b15.is_nan() { f64::NAN } else { if __min_a15 < __min_b15 { __min_a15 } else if __min_b15 < __min_a15 { __min_b15 } else if __min_a15 == 0.0 && __min_b15 == 0.0 { if __min_a15.is_sign_negative() { __min_a15 } else { __min_b15 } } else { __min_a15 } } };
            if match &(next) { Some(__option14) => next_budget.is_some() && (kept) < (right), None => false } {
                updated = PunctuationGeometryLedger::punctuation_geometry_ledger_put_b((updated).clone(), *(next).as_ref().unwrap(), GlueBudget::new((next_budget).as_ref().unwrap().leading_natural, (next_budget).as_ref().unwrap().leading_natural - kept, (next_budget).as_ref().unwrap().trailing_natural, (next_budget).as_ref().unwrap().trailing_consumed));
            }
            let target = { let __min_a16 = 0.0f64 as f64; let __min_b16 = (adjusted - kept) as f64; if __min_a16.is_nan() || __min_b16.is_nan() { f64::NAN } else { if __min_a16 > __min_b16 { __min_a16 } else if __min_b16 > __min_a16 { __min_b16 } else if __min_a16 == 0.0 && __min_b16 == 0.0 { if __min_a16.is_sign_negative() { __min_b16 } else { __min_a16 } } else { __min_a16 } } };
            if target > (0 as f64) {
                trailing = PunctuationGeometryLedger::punctuation_geometry_ledger_put_f((trailing).clone(), end, target);
            }
            if left > (0 as f64) || right != adjusted {
                let n = match &(next) { None => None, Some(__option15) => Some((self.natural_clusters[usize::try_from(*__option15).unwrap_or(0)]).clone()) };
                let reason = match &(next) { None => UString::from("AttachedInlineVirtualPunctuationBoundary:line-end"), Some(__option16) => if match &(left_atom) { Some(__option19) => right_atom != None,
None => false } { UString::from("AttachedInlineVirtualPunctuationBoundary:adjacent-punctuation") } else { if match &(left_atom) { Some(__option21) => __option21.punctuation_class == PunctuationClass::Closing && next_char.is_some() && ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark((next_char).as_deref().unwrap_or(UStr::new(&[]))), None => false } { UString::from("AttachedInlineVirtualPunctuationBoundary:ascii-point-mark") } else { UString::from("AttachedInlineVirtualPunctuationBoundary:natural") }.to_ustring() }.to_ustring() };
                decisions.push(SpacingDecisionInfo::new(TextRange::new(((self.natural_clusters[usize::try_from(previous).unwrap_or(0)]).clone().range).clone().start, u32::from_ne_bytes(((match &(n) { None => ((self.natural_clusters[usize::try_from(end).unwrap_or(0)]).clone().range).clone().end, Some(__option24) => __option24.range.end }) as u32).to_ne_bytes()))?, if i32::from_ne_bytes(((u_string::unit_count(&(((self.natural_clusters[usize::try_from(previous).unwrap_or(0)]).clone().text).to_ustring()))) as i32).to_ne_bytes()) > (0) { u_string::substring(&((self.natural_clusters[usize::try_from(previous).unwrap_or(0)]).clone().text).to_ustring(), i32::from_ne_bytes(((u32::wrapping_sub(u_string::unit_count(&(((self.natural_clusters[usize::try_from(previous).unwrap_or(0)]).clone().text).to_ustring())), 1)) as i32).to_ne_bytes()), i32::wrapping_add(i32::from_ne_bytes(((u32::wrapping_sub(u_string::unit_count(&(((self.natural_clusters[usize::try_from(previous).unwrap_or(0)]).clone().text).to_ustring())), 1)) as i32).to_ne_bytes()), 1)).to_ustring() } else { UString::from(" ") }.as_ustr(), match &(n) { None => UString::from(" "), Some(__option25) => if i32::from_ne_bytes(((u_string::unit_count(&((__option25.text).to_ustring()))) as i32).to_ne_bytes()) > (0) { u_string::substring(&(__option25.text).to_ustring(), 0i32, i32::wrapping_add(0i32, 1)).to_ustring() } else { UString::from(" ") }.to_ustring() }.as_ustr(), natural, adjusted, natural - adjusted, ((self.natural_clusters[usize::try_from(previous).unwrap_or(0)]).clone().range).clone(), reason.as_ustr()));
            }
        }
        let mut attached: SortedMapTable<u32, f64> = PunctuationGeometryLedger::punctuation_geometry_ledger_clone_f((self.attached_inline_trailing_glue_by_cluster).clone());
        for i in 0..u32::from_ne_bytes(((trailing.size()) as u32).to_ne_bytes()) {
            let k = trailing.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            attached = PunctuationGeometryLedger::punctuation_geometry_ledger_put_f((attached).clone(), k, { let __min_a18 = if attached.has(&(k)) { (attached.get(&(k))).unwrap() } else { 0 as f64 } as f64; let __min_b18 = trailing.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) as f64; if __min_a18.is_nan() || __min_b18.is_nan() { f64::NAN } else { if __min_a18 > __min_b18 { __min_a18 } else if __min_b18 > __min_a18 { __min_b18 } else if __min_a18 == 0.0 && __min_b18 == 0.0 { if __min_a18.is_sign_negative() { __min_b18 } else { __min_a18 } } else { __min_a18 } } });
        }
        return Ok(AttachedInlinePunctuationBoundaryResult::new(PunctuationGeometryLedger::new((self.natural_clusters).clone().to_vec(), (self.geometries).clone(), (updated).clone(), Some((self.justification_delta_by_cluster).clone()), Some((self.raw_edge_trim_by_cluster).clone()), Some((self.ruby_spread_by_cluster).clone()), Some((self.inline_box_advance_by_cluster).clone()), Some((attached).clone())), (trailing).clone(), decisions.to_vec()));
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PunctuationGeometryLedger(")); __s += &(UString::from("naturalClusters=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.natural_clusters).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("geometries=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.geometries).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.geometries).clone().key_at(i)) as i32).to_ne_bytes()), (self.geometries).clone().value_at(i).to_string());
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("budgets=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.budgets).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.budgets).clone().key_at(i)) as i32).to_ne_bytes()), (self.budgets).clone().value_at(i).to_string());
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("justificationDeltaByCluster=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.justification_delta_by_cluster).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.justification_delta_by_cluster).clone().key_at(i)) as i32).to_ne_bytes()), (self.justification_delta_by_cluster).clone().value_at(i));
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rawEdgeTrimByCluster=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.raw_edge_trim_by_cluster).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.raw_edge_trim_by_cluster).clone().key_at(i)) as i32).to_ne_bytes()), (self.raw_edge_trim_by_cluster).clone().value_at(i));
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rubySpreadByCluster=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.ruby_spread_by_cluster).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.ruby_spread_by_cluster).clone().key_at(i)) as i32).to_ne_bytes()), (self.ruby_spread_by_cluster).clone().value_at(i));
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineBoxAdvanceByCluster=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.inline_box_advance_by_cluster).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.inline_box_advance_by_cluster).clone().key_at(i)) as i32).to_ne_bytes()), (self.inline_box_advance_by_cluster).clone().value_at(i));
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("attachedInlineTrailingGlueByCluster=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.attached_inline_trailing_glue_by_cluster).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.attached_inline_trailing_glue_by_cluster).clone().key_at(i)) as i32).to_ne_bytes()), (self.attached_inline_trailing_glue_by_cluster).clone().value_at(i));
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub(crate) fn punctuation_geometry_ledger_empty_f() -> SortedMapTable<u32, f64> {
        return SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone().build();
    }

    pub(crate) fn punctuation_geometry_ledger_clone_f(x: SortedMapTable<u32, f64>) -> SortedMapTable<u32, f64> {
        let mut b: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes(((x.size()) as u32).to_ne_bytes()) {
            b.put(&(x.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })), &(x.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        return b.clone().build();
    }

    pub(crate) fn punctuation_geometry_ledger_clone_b(x: SortedMapTable<u32, GlueBudget>) -> SortedMapTable<u32, GlueBudget> {
        let mut b: SortedMapTableBuilder<u32, GlueBudget> = SortedTable::sorted_table_map_builder::<u32, GlueBudget>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes(((x.size()) as u32).to_ne_bytes()) {
            b.put(&(x.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })), &(x.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        return b.clone().build();
    }

    pub(crate) fn punctuation_geometry_ledger_replace(x: PunctuationGeometryLedger, d: Option<SortedMapTable<u32, f64>>, r: Option<SortedMapTable<u32, f64>>, s: Option<SortedMapTable<u32, f64>>, i: Option<SortedMapTable<u32, f64>>, a: Option<SortedMapTable<u32, f64>>, b: Option<SortedMapTable<u32, GlueBudget>>) -> PunctuationGeometryLedger {
        return PunctuationGeometryLedger::new(x.natural_clusters.to_vec(), (x.geometries).clone(), match &(b) { None => (x.budgets).clone(), Some(__option26) => (*__option26).clone() }, Some((match &(d) { None => (x.justification_delta_by_cluster).clone(), Some(__option27) => (*__option27).clone() }).clone()), Some((match &(r) { None => (x.raw_edge_trim_by_cluster).clone(), Some(__option28) => (*__option28).clone() }).clone()), Some((match &(s) { None => (x.ruby_spread_by_cluster).clone(), Some(__option29) => (*__option29).clone() }).clone()), Some((match &(i) { None => (x.inline_box_advance_by_cluster).clone(), Some(__option30) => (*__option30).clone() }).clone()), Some((match &(a) { None => (x.attached_inline_trailing_glue_by_cluster).clone(), Some(__option31) => (*__option31).clone() }).clone()));
    }

    pub fn punctuation_geometry_ledger_from(natural_clusters: &Vec<Cluster>, punctuation_atoms: &Vec<PunctuationAtom>, spacing_plan: PunctuationSpacingCompressionResult) -> PunctuationGeometryLedger {
        let mut geometry_builder: SortedMapTableBuilder<u32, PunctuationClusterGeometry> = SortedTable::sorted_table_map_builder::<u32,
PunctuationClusterGeometry>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from(natural_clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let mut atoms_for_cluster: Vec<PunctuationAtom> = vec![];
            for atom in punctuation_atoms {
                if PunctuationGeometryLedger::punctuation_geometry_ledger_is_inside((atom.range).clone(), ((natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone()) {
                    atoms_for_cluster.push(atom.clone());
                }
            }
            if i32::from_ne_bytes(((u32::try_from((atoms_for_cluster.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                let mut body_terms: Vec<f64> = vec![];
                for atom in &atoms_for_cluster {
                    body_terms.push(atom.body_width);
                }
                let body_width = AccurateSum::accurate_sum_of(&body_terms);
                geometry_builder.put(&(i), &(PunctuationClusterGeometry::new(((natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone(), ((natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring().as_ustr(), ((natural_clusters[usize::try_from(i).unwrap_or(0)]).clone().display_text).to_ustring().as_ustr(), natural_clusters[usize::try_from(i).unwrap_or(0)].advance, body_width, ((atoms_for_cluster[0usize]).clone().leading_glue).clone().natural, ((atoms_for_cluster[usize::try_from(u32::wrapping_sub(u32::try_from((atoms_for_cluster.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().trailing_glue).clone().natural, atoms_for_cluster[0usize].leading_glue_initially_consumed, atoms_for_cluster[usize::try_from(u32::wrapping_sub(u32::try_from((atoms_for_cluster.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].trailing_glue_initially_consumed, if u32::try_from((atoms_for_cluster.len()) & 0xFFFF_FFFF).unwrap_or(0) == 1 { atoms_for_cluster[0usize].glyph_inline_shift } else { 0 as f64 }, if u32::try_from((atoms_for_cluster.len()) & 0xFFFF_FFFF).unwrap_or(0) == 1 { (atoms_for_cluster[0usize]).clone().glyph_placement_reason } else { None }.clone(), if u32::try_from((atoms_for_cluster.len()) & 0xFFFF_FFFF).unwrap_or(0) == 1 { Some(atoms_for_cluster[0usize].anchor) } else { None }, ((atoms_for_cluster[0usize]).clone().geometry_source).to_ustring().as_ustr())));
            }
        }
        let geometries: SortedMapTable<u32, PunctuationClusterGeometry> = geometry_builder.clone().build();
        let mut budget_builder: SortedMapTableBuilder<u32, GlueBudget> = SortedTable::sorted_table_map_builder::<u32,
GlueBudget>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes(((geometries.size()) as u32).to_ne_bytes()) {
            let index = geometries.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            let geometry = geometries.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            budget_builder.put(&(index), &(GlueBudget::new(geometry.leading_glue_natural, geometry.leading_glue_initially_consumed, geometry.trailing_glue_natural, geometry.trailing_glue_initially_consumed)));
        }
        return PunctuationGeometryLedger::new(natural_clusters.to_vec(), (geometries).clone(), budget_builder.clone().build(), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f())).consume_spacing((spacing_plan).clone());
    }

    pub(crate) fn punctuation_geometry_ledger_put_b(m: SortedMapTable<u32, GlueBudget>, k: u32, v: GlueBudget) -> SortedMapTable<u32, GlueBudget> {
        let mut b: SortedMapTableBuilder<u32, GlueBudget> = SortedTable::sorted_table_map_builder::<u32, GlueBudget>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes()) {
            b.put(&(m.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })), &(m.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        b.put(&(k), &(v));
        return b.clone().build();
    }

    pub(crate) fn punctuation_geometry_ledger_put_f(m: SortedMapTable<u32, f64>, k: u32, v: f64) -> SortedMapTable<u32, f64> {
        let mut b: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes(((m.size()) as u32).to_ne_bytes()) {
            b.put(&(m.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })), &(m.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        b.put(&(k), &(v));
        return b.clone().build();
    }

    pub fn punctuation_geometry_ledger_is_inside(self_: TextRange, other: TextRange) -> bool {
        return (i32::from_ne_bytes(((self_.start) as i32).to_ne_bytes())) >= i32::from_ne_bytes(((other.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((self_.end) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((other.end) as i32).to_ne_bytes());
    }

    pub fn punctuation_geometry_ledger_cluster_index_range_for(self_: &Vec<Cluster>, r: TextRange) -> Option<IntRange> {
        if u32::try_from((self_.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return None;
        }
        let mut low = 0u32;
        let mut high = u32::try_from((self_.len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((high) as i32).to_ne_bytes())) {
            let mid = u32::wrapping_add(low, high) >> 1;
            if i32::from_ne_bytes(((((self_[usize::try_from(mid).unwrap_or(0)]).clone().range).clone().start) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((r.start) as i32).to_ne_bytes())) {
                low = u32::wrapping_add(mid, 1);
            } else {
                high = mid;
            }
        }
        let first = low;
        low = first;
        high = u32::try_from((self_.len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((high) as i32).to_ne_bytes())) {
            let mid = u32::wrapping_add(low, high) >> 1;
            if i32::from_ne_bytes(((((self_[usize::try_from(mid).unwrap_or(0)]).clone().range).clone().end) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((r.end) as i32).to_ne_bytes()) {
                low = u32::wrapping_add(mid, 1);
            } else {
                high = mid;
            }
        }
        let last_exclusive = low;
        return if i32::from_ne_bytes(((first) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((last_exclusive) as i32).to_ne_bytes())) { Some(IntRange::new(first, u32::wrapping_sub(last_exclusive, 1))) } else { None };
    }
}

#[derive(Clone, PartialEq)]
pub struct AttachedInlinePunctuationBoundaryResult {
    pub geometry: PunctuationGeometryLedger,
    pub trailing_glue_by_cluster: SortedMapTable<u32, f64>,
    pub decisions: Vec<SpacingDecisionInfo>,
}

impl AttachedInlinePunctuationBoundaryResult {
    pub fn new(geometry: PunctuationGeometryLedger, trailing_glue_by_cluster: SortedMapTable<u32, f64>, decisions: Vec<SpacingDecisionInfo>) -> Self {
        Self {
            geometry,
            trailing_glue_by_cluster,
            decisions,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("AttachedInlinePunctuationBoundaryResult(")); __s += &(UString::from("geometry=")); __s += UString::from(format!("{}", (self.geometry).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("trailingGlueByCluster=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.trailing_glue_by_cluster).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", i32::from_ne_bytes((((self.trailing_glue_by_cluster).clone().key_at(i)) as i32).to_ne_bytes()), (self.trailing_glue_by_cluster).clone().value_at(i));
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("decisions=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationClusterGeometry {
    pub range: TextRange,
    pub source_text: UString,
    pub display_text: UString,
    pub base_advance: f64,
    pub body_width: f64,
    pub leading_glue_natural: f64,
    pub trailing_glue_natural: f64,
    pub leading_glue_initially_consumed: f64,
    pub trailing_glue_initially_consumed: f64,
    pub glyph_inline_shift: f64,
    pub glyph_placement_reason: Option<UString>,
    pub anchor: Option<PunctuationAnchor>,
    pub reason: UString,
}

impl PunctuationClusterGeometry {
    pub fn new(range: TextRange, source_text: &UStr, display_text: &UStr, base_advance: f64, body_width: f64, leading_glue_natural: f64, trailing_glue_natural: f64, leading_glue_initially_consumed: f64, trailing_glue_initially_consumed: f64, glyph_inline_shift: f64, glyph_placement_reason: Option<UString>, anchor: Option<PunctuationAnchor>, reason: &UStr) -> Self {
        Self {
            range,
            source_text: source_text.to_ustring(),
            display_text: display_text.to_ustring(),
            base_advance,
            body_width,
            leading_glue_natural,
            trailing_glue_natural,
            leading_glue_initially_consumed,
            trailing_glue_initially_consumed,
            glyph_inline_shift,
            glyph_placement_reason,
            anchor,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PunctuationClusterGeometry(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("displayText=")); __s += (self.display_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("baseAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.base_advance)); __s += &(UString::from(", ")); __s += &(UString::from("bodyWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.body_width)); __s += &(UString::from(", ")); __s += &(UString::from("leadingGlueNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_glue_natural)); __s += &(UString::from(", ")); __s += &(UString::from("trailingGlueNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_glue_natural)); __s += &(UString::from(", ")); __s += &(UString::from("leadingGlueInitiallyConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_glue_initially_consumed)); __s += &(UString::from(", ")); __s += &(UString::from("trailingGlueInitiallyConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_glue_initially_consumed)); __s += &(UString::from(", ")); __s += &(UString::from("glyphInlineShift=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.glyph_inline_shift)); __s += &(UString::from(", ")); __s += &(UString::from("glyphPlacementReason=")); __s += match &((self.glyph_placement_reason).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("anchor=")); __s += (match &(self.anchor) { None => UString::from("null"), Some(__option32) => UString::from((*__option32).name()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlueBudget {
    pub leading_natural: f64,
    pub leading_consumed: f64,
    pub trailing_natural: f64,
    pub trailing_consumed: f64,
}

impl GlueBudget {
    pub fn new(leading_natural: f64, leading_consumed: f64, trailing_natural: f64, trailing_consumed: f64) -> Self {
        Self {
            leading_natural,
            leading_consumed,
            trailing_natural,
            trailing_consumed,
        }
    }

    pub fn get_leading_remaining(&self) -> f64 {
        return { let __min_a19 = 0.0f64 as f64; let __min_b19 = (self.leading_natural - self.leading_consumed) as f64; if __min_a19.is_nan() || __min_b19.is_nan() { f64::NAN } else { if __min_a19 > __min_b19 { __min_a19 } else if __min_b19 > __min_a19 { __min_b19 } else if __min_a19 == 0.0 && __min_b19 == 0.0 { if __min_a19.is_sign_negative() { __min_b19 } else { __min_a19 } } else { __min_a19 } } };
    }

    pub fn get_trailing_remaining(&self) -> f64 {
        return { let __min_a20 = 0.0f64 as f64; let __min_b20 = (self.trailing_natural - self.trailing_consumed) as f64; if __min_a20.is_nan() || __min_b20.is_nan() { f64::NAN } else { if __min_a20 > __min_b20 { __min_a20 } else if __min_b20 > __min_a20 { __min_b20 } else if __min_a20 == 0.0 && __min_b20 == 0.0 { if __min_a20.is_sign_negative() { __min_b20 } else { __min_a20 } } else { __min_a20 } } };
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("GlueBudget(")); __s += &(UString::from("leadingNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_natural)); __s += &(UString::from(", ")); __s += &(UString::from("leadingConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_consumed)); __s += &(UString::from(", ")); __s += &(UString::from("trailingNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_natural)); __s += &(UString::from(", ")); __s += &(UString::from("trailingConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_consumed)); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, PartialEq)]
pub struct LineEdgeTrimResult {
    pub geometry: PunctuationGeometryLedger,
    pub decisions: Vec<LineEdgeTrimDecisionInfo>,
}

impl LineEdgeTrimResult {
    pub fn new(g: PunctuationGeometryLedger, d: Vec<LineEdgeTrimDecisionInfo>) -> Self {
        Self {
            geometry: g,
            decisions: d,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GlueCapacity {
    pub leading: f64,
    pub trailing: f64,
    pub paired: bool,
}

impl GlueCapacity {
    pub fn new(l: f64, t: f64, p: bool) -> Self {
        Self {
            leading: l,
            trailing: t,
            paired: p,
        }
    }
}
