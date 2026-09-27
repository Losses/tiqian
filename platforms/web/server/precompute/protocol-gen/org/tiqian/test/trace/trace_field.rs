use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct TraceField {
    pub key: String,
    pub value: String,
}

pub fn compare_trace_field(a: &TraceField, b: &TraceField) -> i32 {
    let cmp_key = SortedTable::sorted_table_compare_strings(a.key.as_str(), b.key.as_str());
    if cmp_key != 0 { return cmp_key; }
    let cmp_value = SortedTable::sorted_table_compare_strings(a.value.as_str(), b.value.as_str());
    if cmp_value != 0 { return cmp_value; }
    0
}
