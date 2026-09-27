#![cfg(test)]

use crate::org::tiqian::layout::decide_hyphen_break_test_support::DecideHyphenBreakTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum DecideHyphenBreakTestDiscountsSinoWesternCapacityBeforeChargingCjkLoosenessFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        TestTraceRecorder::new("DecideHyphenBreakTest").section(&"chargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnown");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 4, &DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_cs().unwrap(), 74 as f64,
DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![4]), DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![1]), 8 as f64, None, None), None).unwrap();
    });
}

#[test]
fn discounts_sino_western_capacity_before_charging_cjk_looseness() {
    testlib::run("org.tiqian.layout.DecideHyphenBreakTest.discountsSinoWesternCapacityBeforeChargingCjkLooseness", "org.tiqian.layout.DecideHyphenBreakTest.discountsSinoWesternCapacityBeforeChargingCjkLooseness", || {
        TestTraceRecorder::new("DecideHyphenBreakTest").section(&"discountsSinoWesternCapacityBeforeChargingCjkLooseness");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 4, &DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_cs().unwrap(), 74 as f64,
DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![4]), DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![1]), 8 as f64, Some(DecideHyphenBreakTestSupport::decide_hyphen_break_test_support_m(&vec![2])), Some(4 as f64 as f64)),
None).unwrap();
    });
}
