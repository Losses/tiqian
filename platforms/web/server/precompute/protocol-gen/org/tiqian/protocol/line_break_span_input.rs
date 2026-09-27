use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LineBreakSpanInput {
    pub start: u32,
    pub end: u32,
    pub policy: UString,
}

pub fn compare_line_break_span_input(a: &LineBreakSpanInput, b: &LineBreakSpanInput) -> i32 {
    let cmp_start = if a.start < b.start { -1 } else if a.start > b.start { 1 } else { 0 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = if a.end < b.end { -1 } else if a.end > b.end { 1 } else { 0 };
    if cmp_end != 0 { return cmp_end; }
    let cmp_policy = SortedTable::sorted_table_compare_strings(a.policy.as_ustr(), b.policy.as_ustr());
    if cmp_policy != 0 { return cmp_policy; }
    0
}
