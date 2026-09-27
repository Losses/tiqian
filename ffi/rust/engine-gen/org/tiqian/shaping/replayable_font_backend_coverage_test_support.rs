use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ReplayableFontBackendCoverageTestSupport;

impl ReplayableFontBackendCoverageTestSupport {
    pub fn replayable_font_backend_coverage_test_support_strings(values: &Vec<String>) -> SortedSetTable<String> {
        let mut b: SortedSetTableBuilder<String> = SortedTable::sorted_table_set_builder::<String>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        for value in values {
            b.put(&(value).to_string());
        }
        return b.clone().build();
    }

    pub fn replayable_font_backend_coverage_test_support_roles(values: &Vec<FontRole>) -> &Vec<FontRole> {
        return values;
    }

    pub fn replayable_font_backend_coverage_test_support_axes(key: &str, value: f64) -> SortedMapTable<String, f64> {
        let mut b: SortedMapTableBuilder<String, f64> = SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        b.put(&(key).to_string(), &(value));
        return b.clone().build();
    }
}
