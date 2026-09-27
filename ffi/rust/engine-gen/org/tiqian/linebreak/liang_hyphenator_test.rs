#![cfg(test)]

use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::linebreak::liang_hyphenator::LiangHyphenator;
use crate::org::tiqian::linebreak::parse_tex_hyphenation_patterns::ParseTexHyphenationPatterns;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use std::sync::Arc;


#[test]
fn no_hyphenator_yields_no_opportunities() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.noHyphenatorYieldsNoOpportunities", "org.tiqian.linebreak.LiangHyphenatorTest.noHyphenatorYieldsNoOpportunities", || {
        TestTraceRecorder::new("LiangHyphenatorTest").section(&"noHyphenatorYieldsNoOpportunities");
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &NoHyphenator::new().hyphenate(&"international"), None).unwrap();
    });
}

#[test]
fn odd_level_gap_becomes_a_break_outside_the_margins() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.oddLevelGapBecomesABreakOutsideTheMargins", "org.tiqian.linebreak.LiangHyphenatorTest.oddLevelGapBecomesABreakOutsideTheMargins", || {
        TestTraceRecorder::new("LiangHyphenatorTest").section(&"oddLevelGapBecomesABreakOutsideTheMargins");
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec!["c".to_string()], &vec![(vec![1, 0]).clone()]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![])), Some(1), Some(1));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &h.hyphenate(&"abc"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(&"cab"), None).unwrap();
    });
}

#[test]
fn max_level_wins_and_even_forbids_the_break() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.maxLevelWinsAndEvenForbidsTheBreak", "org.tiqian.linebreak.LiangHyphenatorTest.maxLevelWinsAndEvenForbidsTheBreak", || {
        TestTraceRecorder::new("LiangHyphenatorTest").section(&"maxLevelWinsAndEvenForbidsTheBreak");
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec!["ab".to_string(), "zab".to_string()], &vec![(vec![0, 1, 0]).clone(), (vec![0, 0, 2, 0]).clone()]),
Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![])), Some(1), Some(1));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1], &h.hyphenate(&"ab"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(&"zab"), None).unwrap();
    });
}

#[test]
fn margins_and_short_words_are_respected() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.marginsAndShortWordsAreRespected", "org.tiqian.linebreak.LiangHyphenatorTest.marginsAndShortWordsAreRespected", || {
        TestTraceRecorder::new("LiangHyphenatorTest").section(&"marginsAndShortWordsAreRespected");
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec!["a".to_string()], &vec![(vec![1, 0]).clone()]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![])), Some(2), Some(3));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(&"the"), None).unwrap();
    });
}

#[test]
fn exceptions_override_patterns_and_are_case_insensitive() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.exceptionsOverridePatternsAndAreCaseInsensitive", "org.tiqian.linebreak.LiangHyphenatorTest.exceptionsOverridePatternsAndAreCaseInsensitive", || {
        TestTraceRecorder::new("LiangHyphenatorTest").section(&"exceptionsOverridePatternsAndAreCaseInsensitive");
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec!["table".to_string()], &vec![(vec![2]).clone()])), Some(1), Some(1));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &h.hyphenate(&"table"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &h.hyphenate(&"Table"), None).unwrap();
    });
}

#[test]
fn parses_patterns_and_exception_blocks_stripping_comments() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.parsesPatternsAndExceptionBlocksStrippingComments", "org.tiqian.linebreak.LiangHyphenatorTest.parsesPatternsAndExceptionBlocksStrippingComments", || {
        TestTraceRecorder::new("LiangHyphenatorTest").section(&"parsesPatternsAndExceptionBlocksStrippingComments");
        let p = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(&"\\patterns{ .ach4 a5bal }\\hyphenation{ ta-ble present }");
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 0, 0, 0, 4], &(p.patterns.get(&(".ach").to_string())).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 5, 0, 0, 0], &(p.patterns.get(&("abal").to_string())).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &(p.exceptions.get(&("table").to_string())).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &(p.exceptions.get(&("present").to_string())).as_ref().unwrap(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct LiangHyphenatorTestHelpers;

impl LiangHyphenatorTestHelpers {
    pub fn liang_hyphenator_test_helpers_table(keys: &Vec<String>, values: &Vec<Vec<u32>>) -> SortedMapTable<String, Vec<u32>> {
        let mut b = SortedTable::sorted_table_map_builder::<String, Vec<u32>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((keys.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(keys[usize::try_from(i).unwrap_or(0)]).clone(), &((values[usize::try_from(i).unwrap_or(0)]).clone()));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }
}
