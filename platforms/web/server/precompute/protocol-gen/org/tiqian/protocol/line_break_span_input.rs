use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LineBreakSpanInput {
    pub start: u32,
    pub end: u32,
    pub policy: UString,
}

pub fn compare_line_break_span_input(a: &LineBreakSpanInput, b: &LineBreakSpanInput) -> i32 {
    let cmp_start = match i32::from_ne_bytes((a.start).to_ne_bytes()).cmp(&i32::from_ne_bytes((b.start).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = match i32::from_ne_bytes((a.end).to_ne_bytes()).cmp(&i32::from_ne_bytes((b.end).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_end != 0 { return cmp_end; }
    let cmp_policy = SortedTable::sorted_table_compare_strings(a.policy.as_ustr(), b.policy.as_ustr());
    if cmp_policy != 0 { return cmp_policy; }
    0
}
