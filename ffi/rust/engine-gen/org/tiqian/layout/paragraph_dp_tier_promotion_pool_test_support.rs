use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ParagraphDpTierPromotionPoolTestSupport;

impl ParagraphDpTierPromotionPoolTestSupport {
    pub fn paragraph_dp_tier_promotion_pool_test_support_cluster(index: u32, text: &str, advance: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(index, u32::wrapping_add(index, 1))?, text, "test", advance, Some((text).to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn paragraph_dp_tier_promotion_pool_test_support_han_clusters(n: u32) -> Result<Vec<Cluster>, TextRangeError> {
        let mut _g: Vec<Cluster> = vec![];
        for i in 0..n {
            _g.push(ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_cluster(i, &"中", 16 as f64)?);
        }
        return Ok(_g);
    }

    pub fn paragraph_dp_tier_promotion_pool_test_support_latin_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_cluster(0, &"a", 30 as f64)?).clone(),
    (ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_cluster(1, &"/", 30 as f64)?).clone(),
    (ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_cluster(2, &"b", 25 as f64)?).clone(),
    (ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_cluster(3, &"c", 30 as f64)?).clone(),
    (ParagraphDpTierPromotionPoolTestSupport::paragraph_dp_tier_promotion_pool_test_support_cluster(4, &"d", 30 as f64)?).clone(),
]);
    }

    pub fn paragraph_dp_tier_promotion_pool_test_support_opp(keys: &Vec<u32>, values: &Vec<ProgressiveBreakOpportunity>) -> SortedMapTable<u32, ProgressiveBreakOpportunity> {
        let mut b: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.put(&(keys[usize::try_from(i).unwrap_or(0)]), &((values[usize::try_from(i).unwrap_or(0)]).clone()));
        }
        return b.clone().build();
    }

    pub fn paragraph_dp_tier_promotion_pool_test_support_repair_reason(r: Option<RepairOption>) -> String {
        return match &(r) { None => "".to_string(), Some(__option) => RepairOptions::repair_options_reason((*__option).clone()).to_string() };
    }
}
