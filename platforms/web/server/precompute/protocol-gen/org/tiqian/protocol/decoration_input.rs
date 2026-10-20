use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct DecorationInput {
    pub start: u32,
    pub end: u32,
    pub kind: UString,
}

pub fn compare_decoration_input(a: &DecorationInput, b: &DecorationInput) -> i32 {
    let cmp_start = match i32::from_ne_bytes((a.start).to_ne_bytes()).cmp(&i32::from_ne_bytes((b.start).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = match i32::from_ne_bytes((a.end).to_ne_bytes()).cmp(&i32::from_ne_bytes((b.end).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_end != 0 { return cmp_end; }
    let cmp_kind = SortedTable::sorted_table_compare_strings(a.kind.as_ustr(), b.kind.as_ustr());
    if cmp_kind != 0 { return cmp_kind; }
    0
}
