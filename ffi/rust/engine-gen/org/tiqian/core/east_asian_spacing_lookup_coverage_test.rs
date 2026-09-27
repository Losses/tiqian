#![cfg(test)]

use crate::org::tiqian::core::east_asian_spacing_data::EastAsianSpacingData;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault) -> Self {
        match value {
            EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault) -> Self {
        match value {
            EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault) -> Self {
        match value {
            EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn lookup_covers_every_generated_value_and_both_miss_directions() {
    testlib::run("org.tiqian.core.EastAsianSpacingLookupCoverageTest.lookupCoversEveryGeneratedValueAndBothMissDirections", "org.tiqian.core.EastAsianSpacingLookupCoverageTest.lookupCoversEveryGeneratedValueAndBothMissDirections", || {
        TestTraceRecorder::new("EastAsianSpacingLookupCoverageTest").section(&"lookupCoversEveryGeneratedValueAndBothMissDirections");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Conditional.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(33).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(65).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(48).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(19968).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(40959).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(2).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(1114111).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(34).unwrap().name().to_string().as_str(), None).unwrap();
    });
}
