#![cfg(test)]

use crate::org::tiqian::linebreak::english_hyphenation::EnglishHyphenation;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[test]
fn hyphenates_common_words_at_syllable_points() {
    testlib::run("org.tiqian.linebreak.EnglishHyphenationTest.hyphenatesCommonWordsAtSyllablePoints", "org.tiqian.linebreak.EnglishHyphenationTest.hyphenatesCommonWordsAtSyllablePoints", || {
        TestTraceRecorder::new(&(UStr::new(&[69,110,103,108,105,115,104,72,121,112,104,101,110,97,116,105,111,110,84,101,115,116]))).section(UStr::new(&[104,121,112,104,101,110,97,116,101,115,67,111,109,109,111,110,87,111,114,100,115,65,116,83,121,108,108,97,98,108,101,80,111,105,110,116,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[104,121,45,112,104,101,110,45,97,116,105,111,110]), EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(UStr::new(&[104,121,112,104,101,110,97,116,105,111,110])).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,111,109,45,112,117,116,101,114]), EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(UStr::new(&[99,111,109,112,117,116,101,114])).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((u_string::find_from(&(EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(UStr::new(&[105,110,116,101,114,110,97,116,105,111,110,97,108])).unwrap()), UString::from("in-ter").as_ustr(), 0)) as u32).to_ne_bytes()) == 0, Some((EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_hyphenated(UStr::new(&[105,110,116,101,114,110,97,116,105,111,110,97,108])).unwrap()).to_ustring())).unwrap();
    });
}

#[test]
fn respects_margins_and_short_words() {
    testlib::run("org.tiqian.linebreak.EnglishHyphenationTest.respectsMarginsAndShortWords", "org.tiqian.linebreak.EnglishHyphenationTest.respectsMarginsAndShortWords", || {
        TestTraceRecorder::new(&(UStr::new(&[69,110,103,108,105,115,104,72,121,112,104,101,110,97,116,105,111,110,84,101,115,116]))).section(UStr::new(&[114,101,115,112,101,99,116,115,77,97,114,103,105,110,115,65,110,100,83,104,111,114,116,87,111,114,100,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(UStr::new(&[116,104,101])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(UStr::new(&[97])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(EnglishHyphenationTestHelpers::english_hyphenation_test_helpers_within_margins(&EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(UStr::new(&[115,117,112,101,114,99,97,108,105,102,114,97,103,105,108,105,115,116,105,99])), 2, 3, 20), Some(UString::from("offsets=[2, 5, 8, 13, 17]"))).unwrap();
    });
}

#[test]
fn honours_the_exception_list() {
    testlib::run("org.tiqian.linebreak.EnglishHyphenationTest.honoursTheExceptionList", "org.tiqian.linebreak.EnglishHyphenationTest.honoursTheExceptionList", || {
        TestTraceRecorder::new(&(UStr::new(&[69,110,103,108,105,115,104,72,121,112,104,101,110,97,116,105,111,110,84,101,115,116]))).section(UStr::new(&[104,111,110,111,117,114,115,84,104,101,69,120,99,101,112,116,105,111,110,76,105,115,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(UStr::new(&[112,114,111,106,101,99,116])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![], &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(UStr::new(&[112,114,101,115,101,110,116])), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct EnglishHyphenationTestHelpers;

impl EnglishHyphenationTestHelpers {
    pub fn english_hyphenation_test_helpers_hyphenated(word: &UStr) -> Result<UString, UStringFault> {
    let __units = u_string::units(&word);
    let __count = u_string::unit_count(&word);
        let offsets = EnglishHyphenation::english_hyphenation_en_us()?.hyphenate(word);
        let mut b_b = UString::new();
        let mut i = 0u32;
        let __units1 = u_string::units(&word);
        let __count1 = u_string::unit_count(&word);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) {
            let mut j = 0u32;
            let mut found = false;
            while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((offsets.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                if offsets[usize::try_from(j).unwrap_or(0)] == i {
                    found = true;
                }
                j = u32::wrapping_add(j, 1);
            }
            if found {
                b_b += &(UString::from("-"));
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
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if ({ let v: u32 = values[usize::try_from(i).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((left) as i32).to_ne_bytes())) || ({ let v: u32 = values[usize::try_from(i).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > (i32::from_ne_bytes(((u32::wrapping_sub(length, right)) as i32).to_ne_bytes())) {
                return false;
            }
            i = u32::wrapping_add(i, 1);
        }
        return true;
    }
}
