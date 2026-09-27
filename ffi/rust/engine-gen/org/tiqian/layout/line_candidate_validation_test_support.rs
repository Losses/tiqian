use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct LineCandidateValidationTestSupport;

impl LineCandidateValidationTestSupport {
    pub fn line_candidate_validation_test_support_candidate(hanging: &Vec<u32>, range: Option<IntRange>) -> Result<LineCandidate, TextRangeError> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for &value in hanging {
            b.put(&(value));
        }
        return Ok(LineCandidate::new(match &(range) { None => IntRange::new(0u32, 3u32), Some(__option) => (*__option).clone() }, TextRange::new(0u32, 4u32)?, 64.0f64, 64.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(b.clone().build()))?);
    }
}
