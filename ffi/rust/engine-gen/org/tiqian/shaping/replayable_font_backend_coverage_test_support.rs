use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ReplayableFontBackendCoverageTestSupport;

impl ReplayableFontBackendCoverageTestSupport {
    pub fn replayable_font_backend_coverage_test_support_strings(values: &Vec<UString>) -> SortedSetTable<UString> {
        let mut b: SortedSetTableBuilder<UString> = SortedTable::sorted_table_set_builder::<UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        for value in values {
            b.put(&(value).to_ustring());
        }
        return b.clone().build();
    }

    pub fn replayable_font_backend_coverage_test_support_roles(values: &Vec<FontRole>) -> &Vec<FontRole> {
        return values;
    }

    pub fn replayable_font_backend_coverage_test_support_axes(key: &UStr, value: f64) -> SortedMapTable<UString, f64> {
        let mut b: SortedMapTableBuilder<UString, f64> = SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        b.put(&(key).to_ustring(), &(value));
        return b.clone().build();
    }
}
