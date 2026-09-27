use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct BreakOpportunityDecisionInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub break_offsets: Vec<u32>,
    pub reason: UString,
    pub tier: Option<UString>,
}

impl BreakOpportunityDecisionInfo {
    pub fn new(range: TextRange, source_text: &UStr, break_offsets: Vec<u32>, reason: &UStr, tier: Option<UString>) -> Self {
        let tier = tier.or_else(|| None);
        Self {
            range,
            source_text: source_text.to_ustring(),
            break_offsets,
            reason: reason.to_ustring(),
            tier: tier,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("BreakOpportunityDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("breakOffsets=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.break_offsets).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", i32::from_ne_bytes(((arr[i]) as i32).to_ne_bytes()));
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("tier=")); __s += match &((self.tier).clone()) { Some(v) => v.as_ustr(),
None => UStr::new(&[]) }; __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_break_opportunity_decision_info(a: &BreakOpportunityDecisionInfo, b: &BreakOpportunityDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_ustr(), b.source_text.as_ustr());
    if cmp_source_text != 0 { return cmp_source_text; }
    let mut cmp_break_offsets = 0; for (av, bv) in a.break_offsets.iter().zip(b.break_offsets.iter()) { cmp_break_offsets = match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0,
core::cmp::Ordering::Greater => 1 }; if cmp_break_offsets != 0 { break; } }
    if cmp_break_offsets == 0 { cmp_break_offsets = match a.break_offsets.len().cmp(&b.break_offsets.len()) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; }
    if cmp_break_offsets != 0 { return cmp_break_offsets; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    let cmp_tier = match (&a.tier, &b.tier) { (None, None) => 0, (None, Some(_)) => -1, (Some(_), None) => 1, (Some(av), Some(bv)) => match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 } };
    if cmp_tier != 0 { return cmp_tier; }
    0
}
