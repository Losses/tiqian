#![cfg(test)]

use crate::org::tiqian::layout::decide_hyphen_break_test_support::DecideHyphenBreakTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[derive(Debug, Clone, PartialEq)]
pub enum DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault) -> Self {
        match value {
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault) -> Self {
        match value {
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault) -> Self {
        match value {
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault) -> Self {
        match value {
            DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TracedAssertionsAssertEqualsIntFaultFault(value) => write!(formatter, "{}", value),
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault) -> Self {
        match value {
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault) -> Self {
        match value {
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault) -> Self {
        match value {
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault) -> Self {
        match value {
            DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        DecideHyphenBreakTestChargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnownFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn charges_all_deficit_to_cjk_when_no_sino_western_capacity_is_known() {
    testlib::run("org.tiqian.layout.DecideHyphenBreakTest.chargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnown", "org.tiqian.layout.DecideHyphenBreakTest.chargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnown", || {
        TestTraceRecorder::new(&(UStr::new(&[68,101,99,105,100,101,72,121,112,104,101,110,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[99,104,97,114,103,101,115,65,108,108,68,101,102,105,99,105,116,84,111,67,106,107,87,104,101,110,78,111,83,105,110,111,87,101,115,116,101,114,110,67,97,112,97,99,105,116,121,73,115,75,110,111,119,110]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 4, &DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_cs().unwrap(), 74 as f64, DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![4]), DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![1]), 8 as f64, None, None), None).unwrap();
    });
}

#[test]
fn discounts_sino_western_capacity_before_charging_cjk_looseness() {
    testlib::run("org.tiqian.layout.DecideHyphenBreakTest.discountsSinoWesternCapacityBeforeChargingCjkLooseness", "org.tiqian.layout.DecideHyphenBreakTest.discountsSinoWesternCapacityBeforeChargingCjkLooseness", || {
        TestTraceRecorder::new(&(UStr::new(&[68,101,99,105,100,101,72,121,112,104,101,110,66,114,101,97,107,84,101,115,116]))).section(UStr::new(&[100,105,115,99,111,117,110,116,115,83,105,110,111,87,101,115,116,101,114,110,67,97,112,97,99,105,116,121,66,101,102,111,114,101,67,104,97,114,103,105,110,103,67,106,107,76,111,111,115,101,110,101,115,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 4, &DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_cs().unwrap(), 74 as f64, DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![4]), DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![1]), 8 as f64, Some(DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![2])), Some(4 as f64 as f64)), None).unwrap();
    });
}
