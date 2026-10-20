#![cfg(test)]

use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::linebreak::liang_hyphenator::LiangHyphenator;
use crate::org::tiqian::linebreak::parse_tex_hyphenation_patterns::ParseTexHyphenationPatterns;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[test]
fn no_hyphenator_yields_no_opportunities() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.noHyphenatorYieldsNoOpportunities", "org.tiqian.linebreak.LiangHyphenatorTest.noHyphenatorYieldsNoOpportunities", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,97,110,103,72,121,112,104,101,110,97,116,111,114,84,101,115,116]))).section(UStr::new(&[110,111,72,121,112,104,101,110,97,116,111,114,89,105,101,108,100,115,78,111,79,112,112,111,114,116,117,110,105,116,105,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &NoHyphenator::new().hyphenate(UStr::new(&[105,110,116,101,114,110,97,116,105,111,110,97,108])), None).unwrap();
    });
}

#[test]
fn odd_level_gap_becomes_a_break_outside_the_margins() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.oddLevelGapBecomesABreakOutsideTheMargins", "org.tiqian.linebreak.LiangHyphenatorTest.oddLevelGapBecomesABreakOutsideTheMargins", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,97,110,103,72,121,112,104,101,110,97,116,111,114,84,101,115,116]))).section(UStr::new(&[111,100,100,76,101,118,101,108,71,97,112,66,101,99,111,109,101,115,65,66,114,101,97,107,79,117,116,115,105,100,101,84,104,101,77,97,114,103,105,110,115]));
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![UString::from("c").to_ustring()], &vec![(vec![1, 0]).clone()]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![])), Some(1), Some(1));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &h.hyphenate(UStr::new(&[97,98,99])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(UStr::new(&[99,97,98])), None).unwrap();
    });
}

#[test]
fn max_level_wins_and_even_forbids_the_break() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.maxLevelWinsAndEvenForbidsTheBreak", "org.tiqian.linebreak.LiangHyphenatorTest.maxLevelWinsAndEvenForbidsTheBreak", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,97,110,103,72,121,112,104,101,110,97,116,111,114,84,101,115,116]))).section(UStr::new(&[109,97,120,76,101,118,101,108,87,105,110,115,65,110,100,69,118,101,110,70,111,114,98,105,100,115,84,104,101,66,114,101,97,107]));
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![UString::from("ab").to_ustring(), UString::from("zab").to_ustring()], &vec![(vec![0, 1, 0]).clone(), (vec![0, 0, 2, 0]).clone()]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![])), Some(1), Some(1));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![1], &h.hyphenate(UStr::new(&[97,98])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(UStr::new(&[122,97,98])), None).unwrap();
    });
}

#[test]
fn margins_and_short_words_are_respected() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.marginsAndShortWordsAreRespected", "org.tiqian.linebreak.LiangHyphenatorTest.marginsAndShortWordsAreRespected", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,97,110,103,72,121,112,104,101,110,97,116,111,114,84,101,115,116]))).section(UStr::new(&[109,97,114,103,105,110,115,65,110,100,83,104,111,114,116,87,111,114,100,115,65,114,101,82,101,115,112,101,99,116,101,100]));
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![UString::from("a").to_ustring()], &vec![(vec![1, 0]).clone()]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![])), Some(2), Some(3));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &h.hyphenate(UStr::new(&[116,104,101])), None).unwrap();
    });
}

#[test]
fn exceptions_override_patterns_and_are_case_insensitive() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.exceptionsOverridePatternsAndAreCaseInsensitive", "org.tiqian.linebreak.LiangHyphenatorTest.exceptionsOverridePatternsAndAreCaseInsensitive", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,97,110,103,72,121,112,104,101,110,97,116,111,114,84,101,115,116]))).section(UStr::new(&[101,120,99,101,112,116,105,111,110,115,79,118,101,114,114,105,100,101,80,97,116,116,101,114,110,115,65,110,100,65,114,101,67,97,115,101,73,110,115,101,110,115,105,116,105,118,101]));
        let h = LiangHyphenator::new(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![], &vec![]), Some(LiangHyphenatorTestHelpers::liang_hyphenator_test_helpers_table(&vec![UString::from("table").to_ustring()], &vec![(vec![2]).clone()])), Some(1), Some(1));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &h.hyphenate(UStr::new(&[116,97,98,108,101])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &h.hyphenate(UStr::new(&[84,97,98,108,101])), None).unwrap();
    });
}

#[test]
fn parses_patterns_and_exception_blocks_stripping_comments() {
    testlib::run("org.tiqian.linebreak.LiangHyphenatorTest.parsesPatternsAndExceptionBlocksStrippingComments", "org.tiqian.linebreak.LiangHyphenatorTest.parsesPatternsAndExceptionBlocksStrippingComments", || {
        TestTraceRecorder::new(&(UStr::new(&[76,105,97,110,103,72,121,112,104,101,110,97,116,111,114,84,101,115,116]))).section(UStr::new(&[112,97,114,115,101,115,80,97,116,116,101,114,110,115,65,110,100,69,120,99,101,112,116,105,111,110,66,108,111,99,107,115,83,116,114,105,112,112,105,110,103,67,111,109,109,101,110,116,115]));
        let p = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(UStr::new(&[92,112,97,116,116,101,114,110,115,123,32,46,97,99,104,52,32,97,53,98,97,108,32,125,92,104,121,112,104,101,110,97,116,105,111,110,123,32,116,97,45,98,108,101,32,112,114,101,115,101,110,116,32,125]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 0, 0, 0, 4], &(p.patterns.get(&(UString::from(".ach")).to_ustring())).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 5, 0, 0, 0], &(p.patterns.get(&(UString::from("abal")).to_ustring())).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![2], &(p.exceptions.get(&(UString::from("table")).to_ustring())).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &(p.exceptions.get(&(UString::from("present")).to_ustring())).as_ref().unwrap(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct LiangHyphenatorTestHelpers;

impl LiangHyphenatorTestHelpers {
    pub fn liang_hyphenator_test_helpers_table(keys: &Vec<UString>, values: &Vec<Vec<u32>>) -> SortedMapTable<UString, Vec<u32>> {
        let mut b = SortedTable::sorted_table_map_builder::<UString, Vec<u32>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((keys.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(keys[usize::try_from(i).unwrap_or(0)]).clone(), &((values[usize::try_from(i).unwrap_or(0)]).clone()));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }
}
