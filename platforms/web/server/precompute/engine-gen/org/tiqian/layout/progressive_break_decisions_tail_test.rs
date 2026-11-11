#![cfg(test)]

use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions_tail_test_support::ProgressiveBreakDecisionsTailTestSupport;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsTailTestInfiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTierFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsTailTestInfiniteLineLimitWithClustersAdmitsTheCleanestTierFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn infinite_line_limit_with_clusters_admits_the_cleanest_tier() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsTailTest.infiniteLineLimitWithClustersAdmitsTheCleanestTier", "org.tiqian.layout.ProgressiveBreakDecisionsTailTest.infiniteLineLimitWithClustersAdmitsTheCleanestTier", || {
        ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_t(UStr::new(&[105,110,102,105,110,105,116,101,76,105,110,101,76,105,109,105,116,87,105,116,104,67,108,117,115,116,101,114,115,65,100,109,105,116,115,84,104,101,67,108,101,97,110,101,115,116,84,105,101,114]), {  Arc::new(move || {
        let cs = vec![
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(0).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(1).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(2).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(3).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(4).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 4, ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_o().unwrap(), Some((cs).clone()), None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn infinite_stretch_ceiling_with_finite_line_limit_admits_the_cleanest_tier() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsTailTest.infiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTier", "org.tiqian.layout.ProgressiveBreakDecisionsTailTest.infiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTier", || {
        ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_t(UStr::new(&[105,110,102,105,110,105,116,101,83,116,114,101,116,99,104,67,101,105,108,105,110,103,87,105,116,104,70,105,110,105,116,101,76,105,110,101,76,105,109,105,116,65,100,109,105,116,115,84,104,101,67,108,101,97,110,101,115,116,84,105,101,114]), {  Arc::new(move || {
        let cs = vec![
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(0).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(1).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(2).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(3).unwrap()).clone(),
    (ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_c(4).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 4, ProgressiveBreakDecisionsTailTestSupport::progressive_break_decisions_tail_test_support_o().unwrap(), Some((cs).clone()), Some(200 as f64 as f64), None, None, None, None), None).unwrap();
}) });
    });
}
