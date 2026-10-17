use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutAttribute {
    pub name: UString,
    pub value: UString,
}

pub fn compare_layout_attribute(a: &LayoutAttribute, b: &LayoutAttribute) -> i32 {
    let cmp_name = SortedTable::sorted_table_compare_strings(a.name.as_ustr(), b.name.as_ustr());
    if cmp_name != 0 { return cmp_name; }
    let cmp_value = SortedTable::sorted_table_compare_strings(a.value.as_ustr(), b.value.as_ustr());
    if cmp_value != 0 { return cmp_value; }
    0
}
