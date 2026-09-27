#![cfg(test)]

use crate::org::tiqian::linebreak::english_hyphenation::EnglishHyphenation;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::std::u_string_exception::UStringFault;


#[test]
fn hyphenates_common_words_at_syllable_points() {
    testlib::run("org.tiqian.linebreak.EnglishHyphenationTest.hyphenatesCommonWordsAtSyllablePoints", "org.tiqian.linebreak.EnglishHyphenationTest.hyphenatesCommonWordsAtSyllablePoints", || {
        TestTraceRecorder::new("EnglishHyphenationTest").section(&"hyphenatesCommonWordsAtSyllablePoints");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"hy-phen-ation", EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(&"hyphenation").unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"com-puter", EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(&"computer").unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((u_string::find_from(&EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(&"international").unwrap(), "in-ter", 0)).to_ne_bytes()) == 0,
Some((EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(&"international").unwrap()).to_string())).unwrap();
    });
}

#[test]
fn respects_margins_and_short_words() {
    testlib::run("org.tiqian.linebreak.EnglishHyphenationTest.respectsMarginsAndShortWords", "org.tiqian.linebreak.EnglishHyphenationTest.respectsMarginsAndShortWords", || {
        TestTraceRecorder::new("EnglishHyphenationTest").section(&"respectsMarginsAndShortWords");
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(&"the"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(&"a"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_within_margins(&EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(&"supercalifragilistic"), 2, 3, 20),
Some("offsets=[2, 5, 8, 13, 17]".to_string())).unwrap();
    });
}

#[test]
fn honours_the_exception_list() {
    testlib::run("org.tiqian.linebreak.EnglishHyphenationTest.honoursTheExceptionList", "org.tiqian.linebreak.EnglishHyphenationTest.honoursTheExceptionList", || {
        TestTraceRecorder::new("EnglishHyphenationTest").section(&"honoursTheExceptionList");
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(&"project"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(&"present"), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct EnglishHyphenationTestHelpers;

impl EnglishHyphenationTestHelpers {
    pub fn english_hyphenation_test_helpers_hyphenated(word: &str) -> Result<String, UStringFault> {
    let __units = u_string::units(&word);
    let __count = u_string::unit_count(&word);
        let offsets = EnglishHyphenation::english_hyphenation_en_us()?.hyphenate(word);
        let mut b_b = String::new();
        let mut i = 0u32;
        let __units1 = u_string::units(&word);
        let __count1 = u_string::unit_count(&word);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let mut j = 0u32;
            let mut found = false;
            while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((offsets.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                if offsets[usize::try_from(j).unwrap_or(0)] == i {
                    found = true;
                }
                j = u32::wrapping_add(j, 1);
            }
            if found {
                b_b += &("-");
            }
            {
                let x = u_string::char_at_from(&__units1, i);
                b_b += &(x.to_string());
            }
            i = u32::wrapping_add(i, 1);
        }
        return Ok(b_b);
    }

    pub fn english_hyphenation_test_helpers_within_margins(values: &[u32], left: u32, right: u32, length: u32) -> bool {
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if ({ let v: u32 = values[usize::try_from(i).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((left).to_ne_bytes())) || ({ let v: u32 = values[usize::try_from(i).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) >
(i32::from_ne_bytes((u32::wrapping_sub(length, right)).to_ne_bytes())) {
                return false;
            }
            i = u32::wrapping_add(i, 1);
        }
        return true;
    }
}
