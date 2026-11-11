#![cfg(test)]

use crate::org::tiqian::core::east_asian_spacing_data::EastAsianSpacingData;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingLookupCoverageTestLookupCoversEveryGeneratedValueAndBothMissDirectionsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,76,111,111,107,117,112,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[108,111,111,107,117,112,67,111,118,101,114,115,69,118,101,114,121,71,101,110,101,114,97,116,101,100,86,97,108,117,101,65,110,100,66,111,116,104,77,105,115,115,68,105,114,101,99,116,105,111,110,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Conditional.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(33).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(65).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(48).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(19968).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(40959).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(2).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(1114111).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(34).unwrap().name()).as_ustr(), None).unwrap();
    });
}
