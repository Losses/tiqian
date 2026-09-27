#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreakerLines;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault) -> Self {
        match value {
            LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault) -> Self {
        match value {
            LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault) -> Self {
        match value {
            LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestWindowZeroReducesLookaheadToGreedyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestLookaheadShiftsBreakEarlierToAvoidKinsokuRepairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestLookaheadScoresKinsokuRepairsWithUnbreakableRangesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestLookaheadScoresFuturePushInBeforeChoosingEarlierBreakFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestLookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestLookaheadKeepsGreedyBreakWhenPushInGlueCoversRepairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestLookaheadFallsBackToGreedyWhenAlternativesAreWorseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault) -> Self {
        match value {
            LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestLookaheadAvoidsConsecutiveSyntheticHyphenBreaksFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault) -> Self {
        match value {
            LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault) -> Self {
        match value {
            LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault) -> Self {
        match value {
            LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestHangingTailIsExcludedFromFillDensityGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault) -> Self {
        match value {
            LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault) -> Self {
        match value {
            LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault) -> Self {
        match value {
            LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LookaheadLineBreakerTestHangingClustersMustBeAContiguousTrailingSuffixFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestEmptyInputProducesNoLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestEmptyInputProducesNoLinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestEmptyInputProducesNoLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestEmptyInputProducesNoLinesFault) -> Self {
        match value {
            LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestEmptyInputProducesNoLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestEmptyInputProducesNoLinesFault) -> Self {
        match value {
            LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestEmptyInputProducesNoLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestEmptyInputProducesNoLinesFault) -> Self {
        match value {
            LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestEmptyInputProducesNoLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestEmptyInputProducesNoLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestEmptyInputProducesNoLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestEmptyInputProducesNoLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault) -> Self {
        match value {
            LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault) -> Self {
        match value {
            LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault) -> Self {
        match value {
            LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LookaheadLineBreakerTestCompatibilityHangingIndexSkipsATrailingMandatoryBreakControlFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn hanging_tail_is_excluded_from_fill_density_geometry() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.hangingTailIsExcludedFromFillDensityGeometry", "org.tiqian.layout.LookaheadLineBreakerTest.hangingTailIsExcludedFromFillDensityGeometry", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[104,97,110,103,105,110,103,84,97,105,108,73,115,69,120,99,108,117,100,101,100,70,114,111,109,70,105,108,108,68,101,110,115,105,116,121,71,101,111,109,101,116,114,121]));
        let line = LineCandidate::new(IntRange::new(0u32, 2u32), TextRange::new(0u32, 3u32).unwrap(), 48.0f64, 16.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![1, 2]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), line.get_in_measure_cluster_range(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LineBreakerLines::line_breaker_lines_line_gap_count(line.get_in_measure_cluster_range(), LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![1, 2])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, LineBreakerLines::line_breaker_lines_line_adjustment_density((line).clone(), 48.0f64, false, LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![1, 2])), Some(UString::from("hung point-mark boundaries are not justification gaps"))).unwrap();
    });
}

#[test]
fn hanging_clusters_must_be_a_contiguous_trailing_suffix() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.hangingClustersMustBeAContiguousTrailingSuffix", "org.tiqian.layout.LookaheadLineBreakerTest.hangingClustersMustBeAContiguousTrailingSuffix", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[104,97,110,103,105,110,103,67,108,117,115,116,101,114,115,77,117,115,116,66,101,65,67,111,110,116,105,103,117,111,117,115,84,114,97,105,108,105,110,103,83,117,102,102,105,120]));
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineCandidate::new(IntRange::new(0u32, 2u32), TextRange::new(0u32, 3u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, 48.0f64, 32.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![1]))).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn compatibility_hanging_index_skips_a_trailing_mandatory_break_control() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.compatibilityHangingIndexSkipsATrailingMandatoryBreakControl", "org.tiqian.layout.LookaheadLineBreakerTest.compatibilityHangingIndexSkipsATrailingMandatoryBreakControl", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[99,111,109,112,97,116,105,98,105,108,105,116,121,72,97,110,103,105,110,103,73,110,100,101,120,83,107,105,112,115,65,84,114,97,105,108,105,110,103,77,97,110,100,97,116,111,114,121,66,114,101,97,107,67,111,110,116,114,111,108]));
        let line = LineCandidate::new(IntRange::new(0u32, 2u32), TextRange::new(0u32, 3u32).unwrap(), 32.0f64, 16.0f64, Some(LineEndReason::AutoWrap), Some(RepairOption::Hang { penalty: 5, reason: UString::from("test"), offender_cluster_index: 1 }), Some(vec![]), Some(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![1, 2]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, *(line.get_hanging_cluster_index()).as_ref().unwrap(), None).unwrap();
    });
}

#[test]
fn empty_input_produces_no_lines() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.emptyInputProducesNoLines", "org.tiqian.layout.LookaheadLineBreakerTest.emptyInputProducesNoLines", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[101,109,112,116,121,73,110,112,117,116,80,114,111,100,117,99,101,115,78,111,76,105,110,101,115]));
        let empty_clusters: Vec<Cluster> = vec![];
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&empty_clusters, &empty_clusters, 100.0f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn lookahead_matches_greedy_when_shifting_earlier_gives_no_benefit() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.lookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefit", "org.tiqian.layout.LookaheadLineBreakerTest.lookaheadMatchesGreedyWhenShiftingEarlierGivesNoBenefit", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[108,111,111,107,97,104,101,97,100,77,97,116,99,104,101,115,71,114,101,101,100,121,87,104,101,110,83,104,105,102,116,105,110,103,69,97,114,108,105,101,114,71,105,118,101,115,78,111,66,101,110,101,102,105,116]));
        let mut clusters: Vec<Cluster> = vec![];
        {
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(4, 5, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(5, 6, UStr::new(&[120]), 16.0f64).unwrap());
        }
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 64.0f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 5u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn lookahead_shifts_break_earlier_to_avoid_kinsoku_repair() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.lookaheadShiftsBreakEarlierToAvoidKinsokuRepair", "org.tiqian.layout.LookaheadLineBreakerTest.lookaheadShiftsBreakEarlierToAvoidKinsokuRepair", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[108,111,111,107,97,104,101,97,100,83,104,105,102,116,115,66,114,101,97,107,69,97,114,108,105,101,114,84,111,65,118,111,105,100,75,105,110,115,111,107,117,82,101,112,97,105,114]));
        let clusters = vec![
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(4, 5, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(5, 6, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(6, 7, UStr::new(&[12290]), 16.0f64).unwrap()).clone(),
];
        let greedy = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 48.0f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let lookahead = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 48.0f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((greedy.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(true, LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_is_carry_previous(((greedy.lines[2usize]).clone().repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, greedy.total_badness, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((lookahead.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_repair_option(None, ((lookahead.lines[0usize]).clone().repair).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_repair_option(None, ((lookahead.lines[1usize]).clone().repair).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_repair_option(None, ((lookahead.lines[2usize]).clone().repair).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, lookahead.total_badness, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, lookahead.lines[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48.0f64, lookahead.lines[1usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, lookahead.lines[2usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((lookahead.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 4u32), ((lookahead.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(5u32, 6u32), ((lookahead.lines[2usize]).clone().cluster_range).clone(), None).unwrap();
    });
}

#[test]
fn lookahead_keeps_greedy_break_when_push_in_glue_covers_repair() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.lookaheadKeepsGreedyBreakWhenPushInGlueCoversRepair", "org.tiqian.layout.LookaheadLineBreakerTest.lookaheadKeepsGreedyBreakWhenPushInGlueCoversRepair", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[108,111,111,107,97,104,101,97,100,75,101,101,112,115,71,114,101,101,100,121,66,114,101,97,107,87,104,101,110,80,117,115,104,73,110,71,108,117,101,67,111,118,101,114,115,82,101,112,97,105,114]));
        let clusters = vec![
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[12290]), 16.0f64).unwrap()).clone(),
];
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 60.0f64, Some(vec![(ShrinkOpportunity::new(3u32, 6u32, 4.0f64, ShrinkChannel::TrailingGlue, Some(false))).clone()]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line = (solution.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), (line.cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(60.0f64, line.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(true, LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_is_push_in((line.repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn lookahead_scores_future_push_in_before_choosing_earlier_break() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.lookaheadScoresFuturePushInBeforeChoosingEarlierBreak", "org.tiqian.layout.LookaheadLineBreakerTest.lookaheadScoresFuturePushInBeforeChoosingEarlierBreak", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[108,111,111,107,97,104,101,97,100,83,99,111,114,101,115,70,117,116,117,114,101,80,117,115,104,73,110,66,101,102,111,114,101,67,104,111,111,115,105,110,103,69,97,114,108,105,101,114,66,114,101,97,107]));
        let clusters = vec![
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(4, 5, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(5, 6, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(6, 7, UStr::new(&[12290]), 16.0f64).unwrap()).clone(),
];
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 60.0f64, Some(vec![(ShrinkOpportunity::new(6u32, 6u32, 4.0f64, ShrinkChannel::TrailingGlue, Some(false))).clone()]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(3u32, 6u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(60.0f64, solution.lines[1usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(true, LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_is_push_in(((solution.lines[1usize]).clone().repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn lookahead_falls_back_to_greedy_when_alternatives_are_worse() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.lookaheadFallsBackToGreedyWhenAlternativesAreWorse", "org.tiqian.layout.LookaheadLineBreakerTest.lookaheadFallsBackToGreedyWhenAlternativesAreWorse", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[108,111,111,107,97,104,101,97,100,70,97,108,108,115,66,97,99,107,84,111,71,114,101,101,100,121,87,104,101,110,65,108,116,101,114,110,97,116,105,118,101,115,65,114,101,87,111,114,115,101]));
        let mut clusters: Vec<Cluster> = vec![];
        {
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(4, 5, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(5, 6, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(6, 7, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(7, 8, UStr::new(&[120]), 16.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(8, 9, UStr::new(&[120]), 16.0f64).unwrap());
        }
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 64.0f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 7u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(8u32, 8u32), ((solution.lines[2usize]).clone().cluster_range).clone(), None).unwrap();
    });
}

#[test]
fn lookahead_avoids_consecutive_synthetic_hyphen_breaks() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.lookaheadAvoidsConsecutiveSyntheticHyphenBreaks", "org.tiqian.layout.LookaheadLineBreakerTest.lookaheadAvoidsConsecutiveSyntheticHyphenBreaks", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[108,111,111,107,97,104,101,97,100,65,118,111,105,100,115,67,111,110,115,101,99,117,116,105,118,101,83,121,110,116,104,101,116,105,99,72,121,112,104,101,110,66,114,101,97,107,115]));
        let mut clusters: Vec<Cluster> = vec![];
        {
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[120]), 10.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[120]), 10.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[120]), 10.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[120]), 10.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(4, 5, UStr::new(&[120]), 10.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(5, 6, UStr::new(&[120]), 10.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(6, 7, UStr::new(&[120]), 10.0f64).unwrap());
            clusters.push(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(7, 8, UStr::new(&[120]), 10.0f64).unwrap());
        }
        let no_penalty = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(0.0f64)).break_lines(&clusters, &clusters, 30.0f64, None, None, None, None, None, None, None, Some(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![3, 6])), None, None, None, None, None, None, None, None, None).unwrap();
        let with_penalty = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 30.0f64, None, None, None, None, None, None, None, Some(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![3, 6])), None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((no_penalty.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(3u32, 5u32), ((no_penalty.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(6u32, 7u32), ((no_penalty.lines[2usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((with_penalty.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 4u32), ((with_penalty.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(5u32, 7u32), ((with_penalty.lines[2usize]).clone().cluster_range).clone(), None).unwrap();
    });
}

#[test]
fn lookahead_scores_kinsoku_repairs_with_unbreakable_ranges() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.lookaheadScoresKinsokuRepairsWithUnbreakableRanges", "org.tiqian.layout.LookaheadLineBreakerTest.lookaheadScoresKinsokuRepairsWithUnbreakableRanges", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[108,111,111,107,97,104,101,97,100,83,99,111,114,101,115,75,105,110,115,111,107,117,82,101,112,97,105,114,115,87,105,116,104,85,110,98,114,101,97,107,97,98,108,101,82,97,110,103,101,115]));
        let clusters = vec![
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[30002]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[20057]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[19993]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[19969]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(4, 5, UStr::new(&[25098]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(5, 6, UStr::new(&[24049]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(6, 7, UStr::new(&[24218]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(7, 8, UStr::new(&[36763]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(8, 9, UStr::new(&[12290]), 16.0f64).unwrap()).clone(),
];
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 64.0f64, None, Some(UnbreakableRanges::new(vec![(IntRange::new(6u32, 7u32)).clone()].to_vec())), None, None, None, Some(LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_ints(&vec![8])), None, None, None, None, None, None, Some(false), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 5u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(6u32, 8u32), ((solution.lines[2usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_repair_option(None, ((solution.lines[0usize]).clone().repair).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_repair_option(None, ((solution.lines[1usize]).clone().repair).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_repair_option(None, ((solution.lines[2usize]).clone().repair).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn window_zero_reduces_lookahead_to_greedy() {
    testlib::run("org.tiqian.layout.LookaheadLineBreakerTest.windowZeroReducesLookaheadToGreedy", "org.tiqian.layout.LookaheadLineBreakerTest.windowZeroReducesLookaheadToGreedy", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,111,111,107,97,104,101,97,100,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116])));
        test_trace.section(UStr::new(&[119,105,110,100,111,119,90,101,114,111,82,101,100,117,99,101,115,76,111,111,107,97,104,101,97,100,84,111,71,114,101,101,100,121]));
        let clusters = vec![
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(1, 2, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(2, 3, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(3, 4, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(4, 5, UStr::new(&[20013]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(5, 6, UStr::new(&[25991]), 16.0f64).unwrap()).clone(),
    (LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_cluster(6, 7, UStr::new(&[12290]), 16.0f64).unwrap()).clone(),
];
        let solution = LookaheadLineBreaker::new(Some(0), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 48.0f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(true, LookaheadLineBreakerTestSupport::lookahead_line_breaker_test_support_is_carry_previous(((solution.lines[2usize]).clone().repair).clone()), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct LookaheadLineBreakerTestSupport;

impl LookaheadLineBreakerTestSupport {
    pub fn lookahead_line_breaker_test_support_is_carry_previous(repair: Option<RepairOption>) -> bool {
        if repair == None {
            return false;
        }
        let r = (repair).as_ref().unwrap().clone();
        return match r {
            RepairOption::PushIn { .. } => false,
            RepairOption::Hang { .. } => false,
            RepairOption::CarryPrevious { .. } => true,
            RepairOption::CarryNext { .. } => false,
            RepairOption::LeaveRagged { .. } => false,
        };
    }

    pub fn lookahead_line_breaker_test_support_is_push_in(repair: Option<RepairOption>) -> bool {
        if repair == None {
            return false;
        }
        let r = (repair).as_ref().unwrap().clone();
        return match r {
            RepairOption::PushIn { .. } => true,
            RepairOption::Hang { .. } => false,
            RepairOption::CarryPrevious { .. } => false,
            RepairOption::CarryNext { .. } => false,
            RepairOption::LeaveRagged { .. } => false,
        };
    }

    pub fn lookahead_line_breaker_test_support_cluster(start: u32, end: u32, text: &UStr, advance: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(start, end)?, text, &(UStr::new(&[116,101,115,116])), advance, Some(text.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn lookahead_line_breaker_test_support_ints(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for &v in values {
            b.put(&(v));
        }
        return b.clone().build();
    }
}
