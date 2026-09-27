use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct DecorationInput {
    pub start: u32,
    pub end: u32,
    pub kind: UString,
}

pub fn compare_decoration_input(a: &DecorationInput, b: &DecorationInput) -> i32 {
    let cmp_start = if a.start < b.start { -1 } else if a.start > b.start { 1 } else { 0 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = if a.end < b.end { -1 } else if a.end > b.end { 1 } else { 0 };
    if cmp_end != 0 { return cmp_end; }
    let cmp_kind = SortedTable::sorted_table_compare_strings(a.kind.as_ustr(), b.kind.as_ustr());
    if cmp_kind != 0 { return cmp_kind; }
    0
}
