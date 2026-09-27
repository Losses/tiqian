#![cfg(test)]

use crate::org::tiqian::clreq::number_symbol_cohesion::NumberSymbolCohesion;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn bare_number_is_its_own_group() {
    testlib::run("org.tiqian.clreq.NumberSymbolCohesionTest.bareNumberIsItsOwnGroup", "org.tiqian.clreq.NumberSymbolCohesionTest.bareNumberIsItsOwnGroup", || {
        TestTraceRecorder::new(&(UStr::new(&[78,117,109,98,101,114,83,121,109,98,111,108,67,111,104,101,115,105,111,110,84,101,115,116]))).section(UStr::new(&[98,97,114,101,78,117,109,98,101,114,73,115,73,116,115,79,119,110,71,114,111,117,112]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("2024").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[22312,50,48,50,52,24180])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[32431,20013,25991,27809,26377,25968,23383])), None).unwrap();
    });
}

#[test]
fn binds_digits_with_suffix_unit_prefix_sign_and_currency() {
    testlib::run("org.tiqian.clreq.NumberSymbolCohesionTest.bindsDigitsWithSuffixUnitPrefixSignAndCurrency", "org.tiqian.clreq.NumberSymbolCohesionTest.bindsDigitsWithSuffixUnitPrefixSignAndCurrency", || {
        TestTraceRecorder::new(&(UStr::new(&[78,117,109,98,101,114,83,121,109,98,111,108,67,111,104,101,115,105,111,110,84,101,115,116]))).section(UStr::new(&[98,105,110,100,115,68,105,103,105,116,115,87,105,116,104,83,117,102,102,105,120,85,110,105,116,80,114,101,102,105,120,83,105,103,110,65,110,100,67,117,114,114,101,110,99,121]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("50%").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[22686,38271,53,48,37,20102])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("37℃").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[28201,51,55,8451,39640])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("90°").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[36716,57,48,176,35282])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("+5").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[26159,43,53,24230])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("±2").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[35823,24046,177,50,27627,31859])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("¥100").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[20215,165,49,48,48,20803])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("100₫").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[32422,49,48,48,8363,30340])), None).unwrap();
    });
}

#[test]
fn keeps_interior_decimal_and_thousands_separators() {
    testlib::run("org.tiqian.clreq.NumberSymbolCohesionTest.keepsInteriorDecimalAndThousandsSeparators", "org.tiqian.clreq.NumberSymbolCohesionTest.keepsInteriorDecimalAndThousandsSeparators", || {
        TestTraceRecorder::new(&(UStr::new(&[78,117,109,98,101,114,83,121,109,98,111,108,67,111,104,101,115,105,111,110,84,101,115,116]))).section(UStr::new(&[107,101,101,112,115,73,110,116,101,114,105,111,114,68,101,99,105,109,97,108,65,110,100,84,104,111,117,115,97,110,100,115,83,101,112,97,114,97,116,111,114,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("3.14").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[960,8776,51,46,49,52,21862])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("1,000").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[20849,49,44,48,48,48,20154])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("100").to_ustring()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(UStr::new(&[26377,49,48,48,12290])), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct NumberSymbolCohesionTestHelpers;

impl NumberSymbolCohesionTestHelpers {
    pub fn number_symbol_cohesion_test_helpers_groups(text: &UStr) -> Vec<UString> {
        let ranges = NumberSymbolCohesion::number_symbol_cohesion_unbreakable_ranges(text);
        let mut result: Vec<UString> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            result.push(u_string::substring(&text, i32::from_ne_bytes(((ranges[usize::try_from(index).unwrap_or(0)].start) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_add(ranges[usize::try_from(index).unwrap_or(0)].end, 1)) as i32).to_ne_bytes())));
            index = u32::wrapping_add(index, 1);
        }
        return result;
    }
}
