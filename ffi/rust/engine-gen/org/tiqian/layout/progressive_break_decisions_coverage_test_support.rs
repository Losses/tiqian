use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ProgressiveBreakDecisionsCoverageTestSupport;

impl ProgressiveBreakDecisionsCoverageTestSupport {
    pub fn progressive_break_decisions_coverage_test_support_span() -> Result<TextRange, TextRangeError> {
        return Ok(TextRange::new(0u32, 5u32)?);
    }

    pub fn progressive_break_decisions_coverage_test_support_c(i: u32, text: Option<UString>, a: Option<f64>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, match &(text) { None => UString::from("中"), Some(__option) => __option.to_ustring() }.as_ustr(), &(UStr::new(&[116,101,115,116])), match &(a) { None => 16 as f64, Some(__option1) => *__option1 }, Some((match &(text) { None => UString::from("中"), Some(__option2) => __option2.to_ustring() }).to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn progressive_break_decisions_coverage_test_support_op(t: ProgressiveBreakTier, s: TextRange, cap: Option<f64>) -> ProgressiveBreakOpportunity {
        return ProgressiveBreakOpportunity::new(t, (s).clone(), cap);
    }

    pub fn progressive_break_decisions_coverage_test_support_m(a: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut j = 0u32;
        while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(a[usize::try_from(j).unwrap_or(0)]));
            j = u32::wrapping_add(j, 1);
        }
        return b.clone().build();
    }

    pub fn progressive_break_decisions_coverage_test_support_run(n: &UStr, f: Arc<dyn Fn() -> () + Send + Sync>) {
        TestTraceRecorder::new(&(UStr::new(&[80,114,111,103,114,101,115,115,105,118,101,66,114,101,97,107,68,101,99,105,115,105,111,110,115,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
        f();
    }

    pub fn progressive_break_decisions_coverage_test_support_hy(_n: &UStr, limit: f64, g: SortedSetTable<u32>, s: Option<SortedSetTable<u32>>, cap: Option<f64>) -> Result<u32, TextRangeError> {
        let cs = vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None)?).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None)?).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None)?).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None)?).clone(),
];
        return Ok(ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 3, &cs, limit, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![3]), (g).clone(), 8 as f64, (s).clone(), cap));
    }

    pub fn progressive_break_decisions_coverage_test_support_flush() -> Result<(), UStringFault> {
        let _ = TestTraceRecorder::new(&(UStr::new(&[80,114,111,103,114,101,115,115,105,118,101,66,114,101,97,107,68,101,99,105,115,105,111,110,115,67,111,118,101,114,97,103,101,84,101,115,116]))).flush()?;
        Ok(())
    }
}
