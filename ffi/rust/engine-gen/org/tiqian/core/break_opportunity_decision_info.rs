use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct BreakOpportunityDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub break_offsets: Vec<u32>,
    pub reason: String,
    pub tier: Option<String>,
}

impl BreakOpportunityDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, break_offsets: Vec<u32>, reason: &str, tier: Option<String>) -> Self {
        let tier = tier.or_else(|| None);
        Self {
            range,
            source_text: source_text.to_string(),
            break_offsets,
            reason: reason.to_string(),
            tier: tier,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "BreakOpportunityDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "breakOffsets=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.break_offsets).clone();
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
            "reason=",
            (self.reason).to_string(),
            ", ",
            "tier=",
            match (self.tier).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

pub fn compare_break_opportunity_decision_info(a: &BreakOpportunityDecisionInfo, b: &BreakOpportunityDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_str(), b.source_text.as_str());
    if cmp_source_text != 0 { return cmp_source_text; }
    let mut cmp_break_offsets = 0; for (av, bv) in a.break_offsets.iter().zip(b.break_offsets.iter()) { cmp_break_offsets = match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; if cmp_break_offsets != 0 { break;
} }
    if cmp_break_offsets == 0 { cmp_break_offsets = match a.break_offsets.len().cmp(&b.break_offsets.len()) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; }
    if cmp_break_offsets != 0 { return cmp_break_offsets; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    let cmp_tier = match (&a.tier, &b.tier) { (None, None) => 0, (None, Some(_)) => -1, (Some(_), None) => 1, (Some(av), Some(bv)) => match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 } };
    if cmp_tier != 0 { return cmp_tier; }
    0
}
