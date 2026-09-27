#![cfg(test)]

use crate::org::tiqian::clreq::number_symbol_cohesion::NumberSymbolCohesion;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[test]
fn bare_number_is_its_own_group() {
    testlib::run("org.tiqian.clreq.NumberSymbolCohesionTest.bareNumberIsItsOwnGroup", "org.tiqian.clreq.NumberSymbolCohesionTest.bareNumberIsItsOwnGroup", || {
        TestTraceRecorder::new("NumberSymbolCohesionTest").section(&"bareNumberIsItsOwnGroup");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["2024".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"在2024年"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"纯中文没有数字"), None).unwrap();
    });
}

#[test]
fn binds_digits_with_suffix_unit_prefix_sign_and_currency() {
    testlib::run("org.tiqian.clreq.NumberSymbolCohesionTest.bindsDigitsWithSuffixUnitPrefixSignAndCurrency", "org.tiqian.clreq.NumberSymbolCohesionTest.bindsDigitsWithSuffixUnitPrefixSignAndCurrency", || {
        TestTraceRecorder::new("NumberSymbolCohesionTest").section(&"bindsDigitsWithSuffixUnitPrefixSignAndCurrency");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["50%".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"增长50%了"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["37℃".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"温37℃高"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["90°".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"转90°角"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["+5".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"是+5度"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["±2".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"误差±2毫米"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["¥100".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"价¥100元"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["100₫".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"约100₫的"), None).unwrap();
    });
}

#[test]
fn keeps_interior_decimal_and_thousands_separators() {
    testlib::run("org.tiqian.clreq.NumberSymbolCohesionTest.keepsInteriorDecimalAndThousandsSeparators", "org.tiqian.clreq.NumberSymbolCohesionTest.keepsInteriorDecimalAndThousandsSeparators", || {
        TestTraceRecorder::new("NumberSymbolCohesionTest").section(&"keepsInteriorDecimalAndThousandsSeparators");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["3.14".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"π≈3.14啦"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["1,000".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"共1,000人"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["100".to_string()], &NumberSymbolCohesionTestHelpers::number_symbol_cohesion_test_helpers_groups(&"有100。"), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct NumberSymbolCohesionTestHelpers;

impl NumberSymbolCohesionTestHelpers {
    pub fn number_symbol_cohesion_test_helpers_groups(text: &str) -> Vec<String> {
        let ranges = NumberSymbolCohesion::number_symbol_cohesion_unbreakable_ranges(text);
        let mut result: Vec<String> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            result.push(u_string::substring(&text, i32::from_ne_bytes((ranges[usize::try_from(index).unwrap_or(0)].start).to_ne_bytes()), i32::from_ne_bytes((u32::wrapping_add(ranges[usize::try_from(index).unwrap_or(0)].end, 1)).to_ne_bytes())));
            index = u32::wrapping_add(index, 1);
        }
        return result;
    }
}
