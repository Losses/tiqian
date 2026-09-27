use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::contextual_kinsoku_decision_info::ContextualKinsokuDecisionInfo;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::linebreak::line_break_fns::LineBreakFns;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreak;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreakClass;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub struct SignificantCodePoint {
    pub offset: u32,
    pub code_point: u32,
}

impl SignificantCodePoint {
    pub fn new(offset: u32, code_point: u32) -> Self {
        Self {
            offset,
            code_point,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "SignificantCodePoint(",
            "offset=",
            crate::runtime::int_text::IntText::int_text(self.offset),
            ", ",
            "codePoint=",
            crate::runtime::int_text::IntText::int_text(self.code_point),
            ")"
        );
    }
}

pub fn compare_significant_code_point(a: &SignificantCodePoint, b: &SignificantCodePoint) -> i32 {
    let cmp_offset = if a.offset < b.offset { -1 } else if a.offset > b.offset { 1 } else { 0 };
    if cmp_offset != 0 { return cmp_offset; }
    let cmp_code_point = if a.code_point < b.code_point { -1 } else if a.code_point > b.code_point { 1 } else { 0 };
    if cmp_code_point != 0 { return cmp_code_point; }
    0
}

#[derive(Clone, PartialEq)]
pub struct UnicodePunctuationBoundaries {
    pub forbidden_line_start_clusters: SortedSetTable<u32>,
    pub forbidden_line_end_clusters: SortedSetTable<u32>,
    pub unbreakable_ranges: Vec<IntRange>,
    pub decisions: Vec<ContextualKinsokuDecisionInfo>,
}

impl UnicodePunctuationBoundaries {
    pub fn new(forbidden_line_start_clusters: SortedSetTable<u32>, forbidden_line_end_clusters: SortedSetTable<u32>, unbreakable_ranges: Vec<IntRange>, decisions: Vec<ContextualKinsokuDecisionInfo>) -> Self {
        Self {
            forbidden_line_start_clusters,
            forbidden_line_end_clusters,
            unbreakable_ranges,
            decisions,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "UnicodePunctuationBoundaries(",
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
            "forbiddenLineEndClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.forbidden_line_end_clusters).clone();
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
pub struct AttachedInlineVirtualBoundary {
    pub previous_cluster_index: u32,
    pub attached_cluster_range: IntRange,
    pub next_cluster_index: Option<u32>,
}

impl AttachedInlineVirtualBoundary {
    pub fn new(previous_cluster_index: u32, attached_cluster_range: IntRange, next_cluster_index: Option<u32>) -> Self {
        Self {
            previous_cluster_index,
            attached_cluster_range,
            next_cluster_index,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "AttachedInlineVirtualBoundary(",
            "previousClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.previous_cluster_index),
            ", ",
            "attachedClusterRange=",
            (self.attached_cluster_range).clone().to_string(),
            ", ",
            "nextClusterIndex=",
            match self.next_cluster_index { Some(v) => crate::runtime::int_text::IntText::int_text(v), None => "null".to_string() },
            ")"
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct AttachedInlineInterCharBoundaries {
    pub ordinary_western_boundary_after_clusters: SortedSetTable<u32>,
    pub suppressed_physical_boundary_after_clusters: SortedSetTable<u32>,
    pub virtual_boundary_after_clusters: SortedMapTable<u32, u32>,
    pub virtual_sino_western_boundary_after_clusters: SortedSetTable<u32>,
}

impl AttachedInlineInterCharBoundaries {
    pub fn new(ordinary_western_boundary_after_clusters: SortedSetTable<u32>, suppressed_physical_boundary_after_clusters: SortedSetTable<u32>, virtual_boundary_after_clusters: SortedMapTable<u32, u32>, virtual_sino_western_boundary_after_clusters: SortedSetTable<u32>) -> Self {
        Self {
            ordinary_western_boundary_after_clusters,
            suppressed_physical_boundary_after_clusters,
            virtual_boundary_after_clusters,
            virtual_sino_western_boundary_after_clusters,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "AttachedInlineInterCharBoundaries(",
            "ordinaryWesternBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.ordinary_western_boundary_after_clusters).clone();
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
            "suppressedPhysicalBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.suppressed_physical_boundary_after_clusters).clone();
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
            "virtualBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.virtual_boundary_after_clusters).clone();
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
            "virtualSinoWesternBoundaryAfterClusters=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.virtual_sino_western_boundary_after_clusters).clone();
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
}

#[derive(Clone, Copy)]
pub struct UnicodePunctuationBoundaryResolver;

impl UnicodePunctuationBoundaryResolver {
    pub fn unicode_punctuation_boundary_resolver_resolve_western_bracket_cjk_inter_char_boundaries(text: &str, clusters: &Vec<Cluster>, roles: &Vec<FontRole>) -> Result<SortedSetTable<u32>, TextRangeError> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_western_bracket_cjk_inter_char_boundary(text, &clusters, &roles, i, u32::wrapping_add(i, 1))? {
                b.put(&(i));
            }
            i = u32::wrapping_add(i, 1);
        }
        return Ok(b.clone().build());
    }

    pub fn unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(a: &Vec<InlineAttachment>) -> Vec<AttachedInlineVirtualBoundary> {
        let mut r: Vec<AttachedInlineVirtualBoundary> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if a[usize::try_from(i).unwrap_or(0)] != InlineAttachment::Previous {
                i = u32::wrapping_add(i, 1);
                continue;
            }
            let s = i;
            let mut e = s;
            while (i32::from_ne_bytes((u32::wrapping_add(e, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && a[usize::try_from(u32::wrapping_add(e, 1)).unwrap_or(0)] == InlineAttachment::Previous {
                e = u32::wrapping_add(e, 1);
            }
            if i32::from_ne_bytes((s).to_ne_bytes()) > (0) {
                r.push(AttachedInlineVirtualBoundary::new(u32::wrapping_sub(s, 1), IntRange::new(s, e), if i32::from_ne_bytes((u32::wrapping_add(e, 1)).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
Some(u32::wrapping_add(e, 1)) } else { None }));
            }
            i = u32::wrapping_add(e, 1);
        }
        return r;
    }

    pub fn unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text: &str, clusters: &Vec<Cluster>, roles: &Vec<FontRole>, edges: &Vec<EastAsianSpacingEdges>, western: SortedSetTable<u32>, attachments: &Vec<InlineAttachment>) ->
Result<AttachedInlineInterCharBoundaries, TextRangeError> {
        if u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((roles.len()) & 0xFFFF_FFFF).unwrap_or(0) || u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((edges.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "Clusters, roles and East_Asian_Spacing edges must align.".to_string() });
        }
        if u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((attachments.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return Err(TextRangeError::Message { text: "Inline attachments must align with clusters.".to_string() });
        }
        let vb = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&attachments);
        let mut sup: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((vb.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            sup.put(&(vb[usize::try_from(i).unwrap_or(0)].previous_cluster_index));
            if vb[usize::try_from(i).unwrap_or(0)].next_cluster_index.is_some() {
                sup.put(&((vb[usize::try_from(i).unwrap_or(0)]).clone().attached_cluster_range.end));
            }
            i = u32::wrapping_add(i, 1);
        }
        let sup_set: SortedSetTable<u32> = sup.clone().build();
        let mut ordinary: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((western.size()).to_ne_bytes())).to_ne_bytes())) {
            let x = western.at(i32::from_ne_bytes((i).to_ne_bytes()));
            if !sup_set.has(&(x)) {
                ordinary.put(&(x));
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut vm: SortedMapTableBuilder<u32, u32> = SortedTable::sorted_table_map_builder::<u32, u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut vs: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((vb.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let q = (vb[usize::try_from(i).unwrap_or(0)]).clone();
            i = u32::wrapping_add(i, 1);
            if q.next_cluster_index.is_none() {
                continue;
            }
            let n = q.next_cluster_index;
            let p = q.previous_cluster_index;
            let both = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_cjk(roles[usize::try_from(p).unwrap_or(0)]) &&
UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_cjk(roles[usize::try_from(*((n).as_ref().unwrap())).unwrap_or(0)]);
            let punct = roles[usize::try_from(p).unwrap_or(0)] == FontRole::CjkPunctuation && edges[usize::try_from(*((n).as_ref().unwrap())).unwrap_or(0)].leading == EastAsianSpacingValue::Narrow || edges[usize::try_from(p).unwrap_or(0)].trailing == EastAsianSpacingValue::Narrow
&& roles[usize::try_from(*((n).as_ref().unwrap())).unwrap_or(0)] == FontRole::CjkPunctuation;
            let sino = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_wide_narrow(edges[usize::try_from(p).unwrap_or(0)].trailing, edges[usize::try_from(*((n).as_ref().unwrap())).unwrap_or(0)].leading);
            let bracket = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_western_bracket_cjk_inter_char_boundary(text, &clusters, &roles, p, *(n).as_ref().unwrap())?;
            if both || punct || sino || bracket {
                vm.put(&(q.attached_cluster_range.end), &(p));
            }
            if sino {
                vs.put(&(q.attached_cluster_range.end));
            }
        }
        return Ok(AttachedInlineInterCharBoundaries::new(ordinary.clone().build(), sup.clone().build(), vm.clone().build(), vs.clone().build()));
    }

    pub fn unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text: &str, clusters: &Vec<Cluster>, roles: &Vec<FontRole>, pairs: &Vec<QuotePair>) -> Result<UnicodePunctuationBoundaries, TextRangeError> {
        let mut opens: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut closes: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pairs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            opens.put(&(pairs[usize::try_from(i).unwrap_or(0)].open_index));
            closes.put(&(pairs[usize::try_from(i).unwrap_or(0)].close_index));
            i = u32::wrapping_add(i, 1);
        }
        let open_set: SortedSetTable<u32> = opens.clone().build();
        let close_set: SortedSetTable<u32> = closes.clone().build();
        let mut start: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut end: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut ranges: Vec<IntRange> = vec![];
        let mut decisions: Vec<ContextualKinsokuDecisionInfo> = vec![];
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let c = (clusters[usize::try_from(i).unwrap_or(0)]).clone();
            i = u32::wrapping_add(i, 1);
            let ix = u32::wrapping_sub(i, 1);
            if i32::from_ne_bytes((ix).to_ne_bytes()) >= i32::from_ne_bytes((u32::try_from((roles.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) || roles[usize::try_from(ix).unwrap_or(0)] == FontRole::CjkPunctuation ||
(i32::from_ne_bytes(((c.range).clone().start).to_ne_bytes())) >= i32::from_ne_bytes(((c.range).clone().end).to_ne_bytes()) {
                continue;
            }
            let source = u_string::substring(&text, i32::from_ne_bytes(((c.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((c.range).clone().end).to_ne_bytes()));
            let first = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_first_sig(source.as_str());
            if first.is_none() {
                continue;
            }
            let last = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_last_sig(source.as_str());
            if last.is_none() {
                continue;
            }
            let fc = UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of((first).as_ref().unwrap().code_point)?;
            let lc = UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of((last).as_ref().unwrap().code_point)?;
            let fo = u32::wrapping_add((c.range).clone().start, (first).as_ref().unwrap().offset);
            let lo = u32::wrapping_add((c.range).clone().start, (last).as_ref().unwrap().offset);
            let fdir = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_quote_dir(text, fo, (first).as_ref().unwrap().code_point, fc);
            let ldir = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_quote_dir(text, lo, (last).as_ref().unwrap().code_point, lc);
            let paired_close = close_set.has(&(fo));
            let authored = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_follows(text, fo);
            let forbid_start = if authored { false } else { paired_close || fdir == Dir::Final || fdir == Dir::Unresolved || fc == UnicodePunctuationLineBreakClass::InfixNumericSeparator &&
!UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_decimal_after_space(ix, text, &clusters) || UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_forbid_class(fc) };
            if forbid_start {
                start.put(&(ix));
                let p = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_previous(ix, text, &clusters);
                match &(p) {
                    Some(__option1) => {
                        ranges.push(IntRange::new(*__option1, ix));
                    }
                    None => {
                    }
                }
                let reason = if paired_close { "Uax14WesternPunctuationBoundary:PairedClosingQuote".to_string() } else { if fdir == Dir::Final || fdir == Dir::Unresolved { "Uax14WesternPunctuationBoundary:LB19".to_string() } else { format!("{}{}",
            "Uax14WesternPunctuationBoundary:",
            UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_rule(fc)
        ).to_string() }.to_string() };
                decisions.push(ContextualKinsokuDecisionInfo::new((c.range).clone(), source.as_str(), ix, "LineStart", reason.as_str(), None));
            }
            let paired_open = open_set.has(&(lo));
            let forbid_end = paired_open || ldir == Dir::Initial || ldir == Dir::Unresolved || lc == UnicodePunctuationLineBreakClass::OpenPunctuation;
            if forbid_end {
                end.put(&(ix));
                let n = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_next(ix, text, &clusters);
                match &(n) {
                    Some(__option2) => {
                        ranges.push(IntRange::new(ix, *__option2));
                    }
                    None => {
                    }
                }
                let reason = if paired_open { "Uax14WesternPunctuationBoundary:PairedOpeningQuote".to_string() } else { if ldir == Dir::Initial || ldir == Dir::Unresolved { "Uax14WesternPunctuationBoundary:LB19".to_string() } else {
"Uax14WesternPunctuationBoundary:LB14".to_string() }.to_string() };
                decisions.push(ContextualKinsokuDecisionInfo::new((c.range).clone(), source.as_str(), ix, "LineEnd", reason.as_str(), None));
            }
        }
        return Ok(UnicodePunctuationBoundaries::new(start.clone().build(), end.clone().build(), UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_distinct(&ranges).to_vec(), decisions.to_vec()));
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_is_cjk(r: FontRole) -> bool {
        return r == FontRole::CjkText || r == FontRole::CjkPunctuation;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_wide_narrow(a: EastAsianSpacingValue, b: EastAsianSpacingValue) -> bool {
        return a == EastAsianSpacingValue::Wide && b == EastAsianSpacingValue::Narrow || a == EastAsianSpacingValue::Narrow && b == EastAsianSpacingValue::Wide;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_is_western_bracket_cjk_inter_char_boundary(t: &str, c: &Vec<Cluster>, r: &Vec<FontRole>, a: u32, b: u32) -> Result<bool, TextRangeError> {
    let __units = u_string::units(&t);
    let __count = u_string::unit_count(&t);
        let l = r[usize::try_from(a).unwrap_or(0)] != FontRole::CjkPunctuation && UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_western_cp(UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_last_sig(u_string::substring(&t,
i32::from_ne_bytes((((c[usize::try_from(a).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()), i32::from_ne_bytes((((c[usize::try_from(a).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())).as_str()))?;
        let q = r[usize::try_from(b).unwrap_or(0)] != FontRole::CjkPunctuation && UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_western_cp(UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_first_sig(u_string::substring(&t,
i32::from_ne_bytes((((c[usize::try_from(b).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()), i32::from_ne_bytes((((c[usize::try_from(b).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())).as_str()))?;
        return Ok(l && r[usize::try_from(b).unwrap_or(0)] == FontRole::CjkText || r[usize::try_from(a).unwrap_or(0)] == FontRole::CjkText && q);
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_western_cp(s: Option<SignificantCodePoint>) -> Result<bool, TextRangeError> {
        return Ok(match &(s) { Some(__option3) => UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(__option3.code_point)? == UnicodePunctuationLineBreakClass::OpenPunctuation ||
UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(__option3.code_point)? == UnicodePunctuationLineBreakClass::ClosePunctuation || UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(__option3.code_point)? ==
UnicodePunctuationLineBreakClass::CloseParenthesis, None => false });
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_forbid_class(c: UnicodePunctuationLineBreakClass) -> bool {
        return c == UnicodePunctuationLineBreakClass::ClosePunctuation || c == UnicodePunctuationLineBreakClass::CloseParenthesis || c == UnicodePunctuationLineBreakClass::Exclamation || c == UnicodePunctuationLineBreakClass::InfixNumericSeparator;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_rule(c: UnicodePunctuationLineBreakClass) -> String {
        return if c == UnicodePunctuationLineBreakClass::InfixNumericSeparator { "LB15d".to_string() } else { "LB13".to_string() };
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_decimal_after_space(i: u32, t: &str, c: &Vec<Cluster>) -> bool {
    let __units1 = u_string::units(&t);
    let __count1 = u_string::unit_count(&t);
        if i32::from_ne_bytes((i).to_ne_bytes()) <= 0 {
            return false;
        }
        let p = u_string::substring(&t, i32::from_ne_bytes((((c[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()), i32::from_ne_bytes((((c[usize::try_from(u32::wrapping_sub(i,
1)).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes()));
        if u_string::unit_count(&(p)) == 0 {
            return false;
        }
        let mut j = 0u32;
        let __units2 = u_string::units(&p);
        let __count2 = u_string::unit_count(&p);
        while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) {
            if !UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_ws(*(u_string::unit_at_from(&__units2, j)).as_ref().unwrap()) {
                return false;
            }
            j = u32::wrapping_add(j, 1);
        }
        let cur = u_string::substring(&t, i32::from_ne_bytes((((c[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()), i32::from_ne_bytes((((c[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes()));
        let cp = if i32::from_ne_bytes((u_string::unit_count(&(cur))).to_ne_bytes()) > (1) { UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_at(cur.as_str(), 1) } else { if i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes()) <
(i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_at(u_string::substring(&t, i32::from_ne_bytes((((c[usize::try_from(u32::wrapping_add(i,
1)).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()), i32::from_ne_bytes((((c[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())).as_str(), 0) } else { None } };
        return match &(cp) { Some(__option4) => i32::from_ne_bytes((*__option4).to_ne_bytes()) >= 48 && (i32::from_ne_bytes((*__option4).to_ne_bytes())) <= 57, None => false };
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_quote_dir(t: &str, o: u32, cp: u32, c: UnicodePunctuationLineBreakClass) -> Dir {
        if c != UnicodePunctuationLineBreakClass::Quotation {
            return Dir::None;
        }
        if cp == 8217 {
            let l = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_word(UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_before(t, o));
            let rr = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_word(UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_at(t, u32::wrapping_add(o, 1)));
            return if l && rr { Dir::Word } else { if l { Dir::Final } else { if rr { Dir::Initial } else { Dir::Final } } };
        }
        return if cp == 171 || cp == 8216 || cp == 8219 || cp == 8220 || cp == 8223 || cp == 8249 { Dir::Initial } else { if cp == 187 || cp == 8217 || cp == 8221 || cp == 8250 { Dir::Final } else { Dir::Unresolved } };
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_word(cp: Option<u32>) -> bool {
        return match &(cp) { Some(__option5) => i32::from_ne_bytes((*__option5).to_ne_bytes()) >= 65 && (i32::from_ne_bytes((*__option5).to_ne_bytes())) <= 90 || (i32::from_ne_bytes((*__option5).to_ne_bytes())) >= 97 && (i32::from_ne_bytes((*__option5).to_ne_bytes())) <= 122 ||
(i32::from_ne_bytes((*__option5).to_ne_bytes())) >= 48 && (i32::from_ne_bytes((*__option5).to_ne_bytes())) <= 57 || (i32::from_ne_bytes((*__option5).to_ne_bytes())) >= 192 && (i32::from_ne_bytes((*__option5).to_ne_bytes())) <= 591, None => false };
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_follows(t: &str, o: u32) -> bool {
        let mut x = o;
        while (i32::from_ne_bytes((x).to_ne_bytes())) > (0) {
            let p = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_before(t, x);
            if match &(p) { None => true, Some(__option6) => LineBreakFns::line_break_fns_is_mandatory_break_code_point(*__option6) } || LineBreakFns::line_break_fns_is_zero_width_space_code_point(*(p).as_ref().unwrap()) {
                return true;
            }
            if !UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_ws(*(p).as_ref().unwrap()) {
                return false;
            }
            x = u32::wrapping_sub(x, 1);
        }
        return true;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_is_ws(cp: u32) -> bool {
        return (i32::from_ne_bytes((cp).to_ne_bytes())) <= 65535 && ((i32::from_ne_bytes((cp).to_ne_bytes())) >= 9 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 13 || (i32::from_ne_bytes((cp).to_ne_bytes())) >= 28 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 32 || cp == 160 ||
cp == 5760 || (i32::from_ne_bytes((cp).to_ne_bytes())) >= 8192 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 8202 || cp == 8232 || cp == 8233 || cp == 8239 || cp == 8287 || cp == 12288);
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_previous(i: u32, t: &str, c: &Vec<Cluster>) -> Option<u32> {
        let mut x = i;
        while (x) > (0) {
            let s = u_string::substring(&t, i32::from_ne_bytes((((c[usize::try_from(u32::wrapping_sub(x, 1)).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()), i32::from_ne_bytes((((c[usize::try_from(u32::wrapping_sub(x,
1)).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes()));
            if UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_has_break(s.as_str()) {
                return None;
            }
            if UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_first_sig(s.as_str()).is_some() {
                return Some(u32::wrapping_sub(x, 1));
            }
            x = u32::wrapping_sub(x, 1);
        }
        return None;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_next(i: u32, t: &str, c: &Vec<Cluster>) -> Option<u32> {
        let mut x = u32::wrapping_add(i, 1);
        while (i32::from_ne_bytes((x).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((c.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let s = u_string::substring(&t, i32::from_ne_bytes((((c[usize::try_from(x).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()), i32::from_ne_bytes((((c[usize::try_from(x).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes()));
            if UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_has_break(s.as_str()) {
                return None;
            }
            if UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_first_sig(s.as_str()).is_some() {
                return Some(x);
            }
            x = u32::wrapping_add(x, 1);
        }
        return None;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_has_break(s: &str) -> bool {
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(s))).to_ne_bytes())) {
            let p = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_at(s, i);
            if p.is_none() {
                return false;
            }
            if LineBreakFns::line_break_fns_is_mandatory_break_code_point(*(p).as_ref().unwrap()) || LineBreakFns::line_break_fns_is_zero_width_space_code_point(*(p).as_ref().unwrap()) {
                return true;
            }
            i = u32::wrapping_add(i, if i32::from_ne_bytes((*(p).as_ref().unwrap()).to_ne_bytes()) > (65535) { 2 } else { 1 });
        }
        return false;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_cp_at(s: &str, i: u32) -> Option<u32> {
    let __units3 = u_string::units(&s);
    let __count3 = u_string::unit_count(&s);
        if i > 2147483647 || (i32::from_ne_bytes((i).to_ne_bytes())) >= i32::from_ne_bytes((__count3).to_ne_bytes()) {
            return None;
        }
        let h = u_string::unit_at_from(&__units3, i).unwrap_or(0);
        if i32::from_ne_bytes((h).to_ne_bytes()) < (55296) || (i32::from_ne_bytes((h).to_ne_bytes())) > (56319) || (i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())) >= i32::from_ne_bytes((__count3).to_ne_bytes()) {
            return Some(h);
        }
        let l = u_string::unit_at_from(&__units3, u32::wrapping_add(i, 1)).unwrap_or(0);
        return if i32::from_ne_bytes((l).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((l).to_ne_bytes())) <= 57343 { Some(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(h, 55296)) << (10)), u32::wrapping_sub(l, 56320))) } else { Some(h) };
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_cp_before(s: &str, i: u32) -> Option<u32> {
    let __units4 = u_string::units(&s);
    let __count4 = u_string::unit_count(&s);
        if i32::from_ne_bytes((i).to_ne_bytes()) <= 0 || (i32::from_ne_bytes((i).to_ne_bytes())) > (i32::from_ne_bytes((__count4).to_ne_bytes())) {
            return None;
        }
        let l = u_string::unit_at_from(&__units4, u32::wrapping_sub(i, 1)).unwrap_or(0);
        if i32::from_ne_bytes((l).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((l).to_ne_bytes())) <= 57343 && (i32::from_ne_bytes((i).to_ne_bytes())) > (1) {
            let h = u_string::unit_at_from(&__units4, u32::wrapping_sub(i, 2)).unwrap_or(0);
            if i32::from_ne_bytes((h).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((h).to_ne_bytes())) <= 56319 {
                return Some(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(h, 55296)) << (10)), u32::wrapping_sub(l, 56320)));
            }
        }
        return Some(l);
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_first_sig(s: &str) -> Option<SignificantCodePoint> {
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(s))).to_ne_bytes())) {
            let p = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_at(s, i);
            if p.is_none() {
                return None;
            }
            if !UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_ws(*(p).as_ref().unwrap()) {
                return Some(SignificantCodePoint::new(i, p.unwrap_or(0)));
            }
            i = u32::wrapping_add(i, 1);
        }
        return None;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_last_sig(s: &str) -> Option<SignificantCodePoint> {
    let __units5 = u_string::units(&s);
    let __count5 = u_string::unit_count(&s);
        let mut e = __count5;
        while (i32::from_ne_bytes((e).to_ne_bytes())) > (0) {
            let i = if i32::from_ne_bytes((u_string::unit_at_from(&__units5, u32::wrapping_sub(e, 1)).unwrap_or(0)).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((u_string::unit_at_from(&__units5, u32::wrapping_sub(e, 1)).unwrap_or(0)).to_ne_bytes())) <= 57343 {
u32::wrapping_sub(e, 2) } else { u32::wrapping_sub(e, 1) };
            let p = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_cp_at(s, i);
            if p.is_none() {
                return None;
            }
            if !UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_is_ws(*(p).as_ref().unwrap()) {
                return Some(SignificantCodePoint::new(i, p.unwrap_or(0)));
            }
            e = i;
        }
        return None;
    }

    pub(crate) fn unicode_punctuation_boundary_resolver_distinct(a: &Vec<IntRange>) -> Vec<IntRange> {
        let mut r: Vec<IntRange> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let mut found = false;
            let mut j = 0u32;
            while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                if r[usize::try_from(j).unwrap_or(0)].start == a[usize::try_from(i).unwrap_or(0)].start && r[usize::try_from(j).unwrap_or(0)].end == a[usize::try_from(i).unwrap_or(0)].end {
                    found = true;
                }
                j = u32::wrapping_add(j, 1);
            }
            if !found {
                r.push((a[usize::try_from(i).unwrap_or(0)]).clone());
            }
            i = u32::wrapping_add(i, 1);
        }
        return r;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    Initial,
    Final,
    Unresolved,
    Word,
    None,
}

pub fn compare_dir(a: &Dir, b: &Dir) -> i32 {
    if a == b { return 0; }
    fn rank(v: &Dir) -> i32 {
        match v {
            Dir::Initial => 0,
            Dir::Final => 1,
            Dir::Unresolved => 2,
            Dir::Word => 3,
            Dir::None => 4,
        }
    }
    rank(a) - rank(b)
}

impl Dir {
    pub fn to_string(&self) -> String {
        match self {
            Dir::Initial => "Initial".to_string(),
            Dir::Final => "Final".to_string(),
            Dir::Unresolved => "Unresolved".to_string(),
            Dir::Word => "Word".to_string(),
            Dir::None => "None".to_string(),
        }
    }
}
