use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ProgressiveBreakDecisionsTailTestSupport;

impl ProgressiveBreakDecisionsTailTestSupport {
    pub fn progressive_break_decisions_tail_test_support_c(i: u32) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, "中", "test", 16 as f64 as f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn progressive_break_decisions_tail_test_support_o() -> Result<SortedMapTable<u32, ProgressiveBreakOpportunity>, TextRangeError> {
        let mut b: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        let s = TextRange::new(0u32, 5u32)?;
        b.put(&(2), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Whitespace, (s).clone(), Some(0.0))));
        b.put(&(4), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (s).clone(), Some(0.0))));
        return Ok(b.clone().build());
    }

    pub fn progressive_break_decisions_tail_test_support_t(n: &str, f: Arc<dyn Fn() -> () + Send + Sync>) {
        TestTraceRecorder::new("ProgressiveBreakDecisionsTailTest").section(n);
        f();
    }

    pub fn progressive_break_decisions_tail_test_support_flush() -> Result<(), UStringFault> {
        let _ = TestTraceRecorder::new("ProgressiveBreakDecisionsTailTest").flush()?;
        Ok(())
    }
}
