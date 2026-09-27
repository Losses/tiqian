use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct DecideHyphenBreakTestSupport;

impl DecideHyphenBreakTestSupport {
    pub fn decide_hyphen_break_test_support_c(i: u32, a: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, &(UStr::new(&[120])), &(UStr::new(&[107])), a, Some(UString::from("x")), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn decide_hyphen_break_test_support_cs() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_c(0, 16 as f64)?).clone(),
    (DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_c(1, 16 as f64)?).clone(),
    (DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_c(2, 32 as f64)?).clone(),
    (DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_c(3, 32 as f64)?).clone(),
    (DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_c(4, 32 as f64)?).clone(),
]);
    }

    pub fn decide_hyphen_break_test_support_m(a: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut j = 0u32;
        while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(a[usize::try_from(j).unwrap_or(0)]));
            j = u32::wrapping_add(j, 1);
        }
        return b.clone().build();
    }

    pub fn decide_hyphen_break_test_support_flush() -> Result<(), UStringFault> {
        let _ = TestTraceRecorder::new(&(UStr::new(&[68,101,99,105,100,101,72,121,112,104,101,110,66,114,101,97,107,84,101,115,116]))).flush()?;
        Ok(())
    }
}
