use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ProgressiveTechnicalBreakTestSupport;

impl ProgressiveTechnicalBreakTestSupport {
    pub fn progressive_technical_break_test_support_cluster(index: u32, text: &str, advance: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(index, u32::wrapping_add(index, 1))?, text, "test", advance, Some((text).to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn progressive_technical_break_test_support_opportunity_map(keys: &Vec<u32>, values: &Vec<ProgressiveBreakOpportunity>) -> SortedMapTable<u32, ProgressiveBreakOpportunity> {
        let mut b: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.put(&(keys[usize::try_from(i).unwrap_or(0)]), &((values[usize::try_from(i).unwrap_or(0)]).clone()));
        }
        return b.clone().build();
    }
}
