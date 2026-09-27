use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct TraceField {
    pub key: UString,
    pub value: UString,
}

pub fn compare_trace_field(a: &TraceField, b: &TraceField) -> i32 {
    let cmp_key = SortedTable::sorted_table_compare_strings(a.key.as_ustr(), b.key.as_ustr());
    if cmp_key != 0 { return cmp_key; }
    let cmp_value = SortedTable::sorted_table_compare_strings(a.value.as_ustr(), b.value.as_ustr());
    if cmp_value != 0 { return cmp_value; }
    0
}
