#![cfg(test)]

use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::paragraph_dp_line_breaker_test_support::ParagraphDpLineBreakerTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTilesAllClustersInOrderFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestTilesAllClustersInOrderFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTilesAllClustersInOrderFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestTilesAllClustersInOrderFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTilesAllClustersInOrderFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestTilesAllClustersInOrderFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn compressed_same_tier_boundary_is_not_reported_as_promotion() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.compressedSameTierBoundaryIsNotReportedAsPromotion", "org.tiqian.layout.ParagraphDpLineBreakerTest.compressedSameTierBoundaryIsNotReportedAsPromotion", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[99,111,109,112,114,101,115,115,101,100,83,97,109,101,84,105,101,114,66,111,117,110,100,97,114,121,73,115,78,111,116,82,101,112,111,114,116,101,100,65,115,80,114,111,109,111,116,105,111,110]));
        let cs = vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, UStr::new(&[97]), 30 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, UStr::new(&[47]), 30 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(2, UStr::new(&[98]), 25 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, UStr::new(&[99]), 30 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(4, UStr::new(&[100]), 30 as f64).unwrap()).clone(),
];
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve(&cs, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None,
Some(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_opportunities(&vec![2, 3], &vec![(TextRange::new(0u32, 5u32).unwrap()).clone(), (TextRange::new(0u32, 5u32).unwrap()).clone()], &vec![ProgressiveBreakTier::Emergency, ProgressiveBreakTier::Emergency])),
None, Some(vec![]), Some(8.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(match &((s.lines[0usize]).clone().repair) { Some(__option1) => (RepairOptions::repair_options_reason((*__option1).clone())).starts_with(&UString::from("LineAdjustmentPushIn")), None => false }, None).unwrap();
    });
}

#[test]
fn tiles_all_clusters_in_order() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.tilesAllClustersInOrder", "org.tiqian.layout.ParagraphDpLineBreakerTest.tilesAllClustersInOrder", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[116,105,108,101,115,65,108,108,67,108,117,115,116,101,114,115,73,110,79,114,100,101,114]));
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(23, None).unwrap(), 100 as f64, None, None, None, None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 23).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[80,97,114,97,103,114,97,112,104,69,110,100]), UString::from(s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].end_reason.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn single_line_when_everything_fits() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.singleLineWhenEverythingFits", "org.tiqian.layout.ParagraphDpLineBreakerTest.singleLineWhenEverythingFits", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[115,105,110,103,108,101,76,105,110,101,87,104,101,110,69,118,101,114,121,116,104,105,110,103,70,105,116,115]));
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(4, None).unwrap(), 400 as f64, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[80,97,114,97,103,114,97,112,104,69,110,100]), UString::from(s.lines[0usize].end_reason.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn mandatory_break_binds_control_to_previous_line() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.mandatoryBreakBindsControlToPreviousLine", "org.tiqian.layout.ParagraphDpLineBreakerTest.mandatoryBreakBindsControlToPreviousLine", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[109,97,110,100,97,116,111,114,121,66,114,101,97,107,66,105,110,100,115,67,111,110,116,114,111,108,84,111,80,114,101,118,105,111,117,115,76,105,110,101]));
        let cs = vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, UStr::new(&[20013]), 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, UStr::new(&[20013]), 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(2, UStr::new(&[10]), 0 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, UStr::new(&[20013]), 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(4, UStr::new(&[20013]), 16 as f64).unwrap()).clone(),
];
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 200 as f64, None, Some(vec![2]), None, None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 5).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107]), UString::from(s.lines[0usize].end_reason.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[80,97,114,97,103,114,97,112,104,69,110,100]), UString::from(s.lines[1usize].end_reason.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn trailing_mandatory_break_emits_paragraph_end_line() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.trailingMandatoryBreakEmitsParagraphEndLine", "org.tiqian.layout.ParagraphDpLineBreakerTest.trailingMandatoryBreakEmitsParagraphEndLine", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[116,114,97,105,108,105,110,103,77,97,110,100,97,116,111,114,121,66,114,101,97,107,69,109,105,116,115,80,97,114,97,103,114,97,112,104,69,110,100,76,105,110,101]));
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, UStr::new(&[20013]), 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, UStr::new(&[10]), 0 as f64).unwrap()).clone(),
], 200 as f64, None, Some(vec![1]), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107]), UString::from(s.lines[0usize].end_reason.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[80,97,114,97,103,114,97,112,104,69,110,100]), UString::from(s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].end_reason.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((((s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().cluster_range.start) as i32).to_ne_bytes())) > (i32::from_ne_bytes((((s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().cluster_range.end) as i32).to_ne_bytes())), None).unwrap();
    });
}

#[test]
fn never_breaks_inside_unbreakable_range() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.neverBreaksInsideUnbreakableRange", "org.tiqian.layout.ParagraphDpLineBreakerTest.neverBreaksInsideUnbreakableRange", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[110,101,118,101,114,66,114,101,97,107,115,73,110,115,105,100,101,85,110,98,114,101,97,107,97,98,108,101,82,97,110,103,101]));
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(10, None).unwrap(), 64 as f64, None, None, None, Some(UnbreakableRanges::new(vec![(IntRange::new(3u32, 6u32)).clone()].to_vec())), None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 10).unwrap();
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if i32::from_ne_bytes((((l.cluster_range).clone().start) as i32).to_ne_bytes()) <= i32::from_ne_bytes((((l.cluster_range).clone().end) as i32).to_ne_bytes()) {
                    let _ = TracedAssertions::traced_assertions_assert_true(!((i32::from_ne_bytes((((l.cluster_range).clone().start) as i32).to_ne_bytes())) >= 4 && (i32::from_ne_bytes((((l.cluster_range).clone().start) as i32).to_ne_bytes())) <= 6), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("break inside unbreakable range: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((l.cluster_range).clone().start)).as_str())); __s += &(UString::from("..")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((l.cluster_range).clone().end)).as_str())); __s }).as_str()))).unwrap();
                }
            }
        }
    });
}

#[test]
fn kinsoku_avoidance_routes_around_forbidden_line_start() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.kinsokuAvoidanceRoutesAroundForbiddenLineStart", "org.tiqian.layout.ParagraphDpLineBreakerTest.kinsokuAvoidanceRoutesAroundForbiddenLineStart", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[107,105,110,115,111,107,117,65,118,111,105,100,97,110,99,101,82,111,117,116,101,115,65,114,111,117,110,100,70,111,114,98,105,100,100,101,110,76,105,110,101,83,116,97,114,116]));
        let mut cs = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(7, None).unwrap();
        cs[6usize] = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(6, UStr::new(&[12290]), 16 as f64).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 48 as f64, None, None, None, None, None, Some(vec![6])).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 7).unwrap();
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if i32::from_ne_bytes((((l.cluster_range).clone().start) as i32).to_ne_bytes()) <= i32::from_ne_bytes((((l.cluster_range).clone().end) as i32).to_ne_bytes()) {
                    let _ = TracedAssertions::traced_assertions_assert_true((l.cluster_range).clone().start != 6 || (l.repair).clone() != None, Some(UString::from("。 must not start a line without a recorded repair"))).unwrap();
                }
            }
        }
    });
}

#[test]
fn compression_edge_records_push_in_repair() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.compressionEdgeRecordsPushInRepair", "org.tiqian.layout.ParagraphDpLineBreakerTest.compressionEdgeRecordsPushInRepair", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[99,111,109,112,114,101,115,115,105,111,110,69,100,103,101,82,101,99,111,114,100,115,80,117,115,104,73,110,82,101,112,97,105,114]));
        let mut cs = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(7, None).unwrap();
        cs[3usize] = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, UStr::new(&[65292]), 16 as f64).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 56 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 5u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, Some(true), None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 7).unwrap();
        let mut compressed: Option<LineCandidate> = None;
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if compressed.is_none() && ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_push_in_reason((l.repair).clone()).is_some() {
                    compressed = Some(l.clone());
                }
            }
        }
        if compressed.is_none() {
            let _ = TracedAssertions::traced_assertions_assert_true(false, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("expected a PushIn-compressed line, got ")); __s += ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone()).as_ustr(); __s }).as_str()))).unwrap();
            return;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((compressed).as_ref().unwrap().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((compressed).as_ref().unwrap().adjusted_width) <= 56.01f64, Some(UString::from("compressed line must fit the measure"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_push_in_reason(((compressed).as_ref().unwrap().repair).clone())).as_ref().unwrap()).starts_with(&UString::from("LineAdjustmentPushIn")), Some(UString::from("compression must be recorded as the fill-pass reason code"))).unwrap();
    });
}

#[test]
fn compression_disabled_without_push_in_flag() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.compressionDisabledWithoutPushInFlag", "org.tiqian.layout.ParagraphDpLineBreakerTest.compressionDisabledWithoutPushInFlag", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[99,111,109,112,114,101,115,115,105,111,110,68,105,115,97,98,108,101,100,87,105,116,104,111,117,116,80,117,115,104,73,110,70,108,97,103]));
        let mut cs = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(5, None).unwrap();
        cs[3usize] = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, UStr::new(&[65292]), 16 as f64).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 56 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 5u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, Some(false), None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 5).unwrap();
        let mut none = true;
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                let r = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_push_in_reason((l.repair).clone());
                match &(r) {
                    Some(__option2) => {
                        if __option2.starts_with(&UString::from("LineAdjustmentPushIn")) {
                        none = false;
                        }
                    }
                    None => {
                    }
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, Some(UString::from("PushOutOnly must not produce fill push-ins"))).unwrap();
    });
}

#[test]
fn over_wide_single_cluster_still_progresses() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.overWideSingleClusterStillProgresses", "org.tiqian.layout.ParagraphDpLineBreakerTest.overWideSingleClusterStillProgresses", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(UStr::new(&[111,118,101,114,87,105,100,101,83,105,110,103,108,101,67,108,117,115,116,101,114,83,116,105,108,108,80,114,111,103,114,101,115,115,101,115]));
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, UStr::new(&[20013]), 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, UStr::new(&[65335]), 300 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(2, UStr::new(&[20013]), 16 as f64).unwrap()).clone(),
], 48 as f64, None, None, None, None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 3).unwrap();
    });
}
