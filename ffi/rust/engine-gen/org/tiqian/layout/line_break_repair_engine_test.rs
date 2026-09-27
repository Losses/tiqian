#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_break_repair_engine_test_support::LineBreakRepairEngineTestSupport;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_dp_line_breaker::ParagraphDpLineBreaker;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::linebreak::hyphenator::SyllableHyphenator;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault) -> Self {
        match value {
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault) -> Self {
        match value {
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault) -> Self {
        match value {
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault) -> Self {
        match value {
            LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestUnbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTrackingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalHardBreakOverridesNumberRunCohesionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasureFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueTokenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault) -> Self {
        match value {
            LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestProgressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestOverlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestOverlongLatinWordHardBreaksWithAHangingHyphenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault) -> Self {
        match value {
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault) -> Self {
        match value {
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault) -> Self {
        match value {
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault) -> Self {
        match value {
            LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestOpaqueLatinTokenAfterCjkPullsPrefixOntoLooseLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestLongAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestLatinSolidusBreaksAfterSlashWithoutAddingHyphenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault) -> Self {
        match value {
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault) -> Self {
        match value {
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault) -> Self {
        match value {
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault) -> Self {
        match value {
            LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestHyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOneFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault) -> Self {
        match value {
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault) -> Self {
        match value {
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault) -> Self {
        match value {
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault) -> Self {
        match value {
            LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestGreedyBreakerProducesMultipleLinesWhenWidthOverflowsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault) -> Self {
        match value {
            LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestCamelCaseTokenBreaksAtTheHumpWithoutAHyphenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault) -> Self {
        match value {
            LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestAllCapsAbbreviationIsNeverBrokenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn all_caps_abbreviation_is_never_broken() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.allCapsAbbreviationIsNeverBroken", "org.tiqian.layout.LineBreakRepairEngineTest.allCapsAbbreviationIsNeverBroken", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,108,108,67,97,112,115,65,98,98,114,101,118,105,97,116,105,111,110,73,115,78,101,118,101,114,66,114,111,107,101,110]));
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UStr::new(&[73,78,84,69,82,78,65,84,73,79,78,65,76,73,90,65,84,73,79,78,20013]), 128 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[73,78,84,69,82,78,65,84,73,79,78,65,76,73,90,65,84,73,79,78])), None).unwrap();
    });
}

#[test]
fn camel_case_token_breaks_at_the_hump_without_a_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.camelCaseTokenBreaksAtTheHumpWithoutAHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.camelCaseTokenBreaksAtTheHumpWithoutAHyphen", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[99,97,109,101,108,67,97,115,101,84,111,107,101,110,66,114,101,97,107,115,65,116,84,104,101,72,117,109,112,87,105,116,104,111,117,116,65,72,121,112,104,101,110]));
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UStr::new(&[80,111,119,101,114,80,111,105,110,116]), 128 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[80,111,119,101,114])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[80,111,105,110,116])), None).unwrap();
    });
}

#[test]
fn greedy_breaker_produces_multiple_lines_when_width_overflows() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.greedyBreakerProducesMultipleLinesWhenWidthOverflows", "org.tiqian.layout.LineBreakRepairEngineTest.greedyBreakerProducesMultipleLinesWhenWidthOverflows", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[103,114,101,101,100,121,66,114,101,97,107,101,114,80,114,111,100,117,99,101,115,77,117,108,116,105,112,108,101,76,105,110,101,115,87,104,101,110,87,105,100,116,104,79,118,101,114,102,108,111,119,115]));
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UStr::new(&[20013,25991,25490,29256,24341,25806,27979,35797]), 64 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(8, u32::try_from((r.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let a = (r.lines[0usize]).clone();
        let b = (r.lines[1usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, (a.range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, (a.range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, a.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24 as f64, a.bottom, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, (b.range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(8, (b.range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, b.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24 as f64, b.top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, b.bottom, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from(((r.debug).clone().line_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut ok = true;
        for i in 0..match u32::try_from((r.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().kind.to_ustring() != UString::from("greedy") {
                ok = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, (r.size).clone().height, None).unwrap();
    });
}

#[test]
fn hyphenated_compound_breaks_at_existing_hyphen_without_adding_one() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.hyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOne", "org.tiqian.layout.LineBreakRepairEngineTest.hyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOne", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[104,121,112,104,101,110,97,116,101,100,67,111,109,112,111,117,110,100,66,114,101,97,107,115,65,116,69,120,105,115,116,105,110,103,72,121,112,104,101,110,87,105,116,104,111,117,116,65,100,100,105,110,103,79,110,101]));
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UStr::new(&[111,117,116,45,111,102,45,116,104,101,45,119,97,121]), 128 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[111,117,116,45])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[119,97,121])), None).unwrap();
        let mut x = UString::new();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes(((((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((((r.lines[0usize]).clone().range).clone().end) as i32).to_ne_bytes()) {
                x = ((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring();
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((x).ends_with(&UString::from("-")), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("line 0 should end at the existing hyphen: ")); __s += x.as_ustr(); __s }).as_str()))).unwrap();
    });
}

#[test]
fn latin_solidus_breaks_after_slash_without_adding_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.latinSolidusBreaksAfterSlashWithoutAddingHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.latinSolidusBreaksAfterSlashWithoutAddingHyphen", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,97,116,105,110,83,111,108,105,100,117,115,66,114,101,97,107,115,65,102,116,101,114,83,108,97,115,104,87,105,116,104,111,117,116,65,100,100,105,110,103,72,121,112,104,101,110]));
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(UStr::new(&[84,101,88,47,76,97,84,101,88]), 80 as f64, false, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[84,101,88,47])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[76,97,84,101,88])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[84,101,88,47]), LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,97,84,101,88]), LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 1).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
    });
}

#[test]
fn long_all_caps_opaque_token_hard_breaks_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.longAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.longAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,111,110,103,65,108,108,67,97,112,115,79,112,97,113,117,101,84,111,107,101,110,72,97,114,100,66,114,101,97,107,115,87,105,116,104,111,117,116,83,121,110,116,104,101,116,105,99,72,121,112,104,101,110]));
        let s = UString::from("QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo").to_ustring();
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(s.as_ustr(), 96 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), s.as_ustr()), None).unwrap();
    });
}

#[test]
fn long_letter_blob_stays_opaque_even_when_tail_looks_hyphenatable() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.longLetterBlobStaysOpaqueEvenWhenTailLooksHyphenatable", "org.tiqian.layout.LineBreakRepairEngineTest.longLetterBlobStaysOpaqueEvenWhenTailLooksHyphenatable", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,111,110,103,76,101,116,116,101,114,66,108,111,98,83,116,97,121,115,79,112,97,113,117,101,69,118,101,110,87,104,101,110,84,97,105,108,76,111,111,107,115,72,121,112,104,101,110,97,116,97,98,108,101]));
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_blob_test_tail((t).clone()).unwrap();
    });
}

#[test]
fn long_opaque_token_can_break_even_when_it_fits_alone_but_not_after_cjk_prefix() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.longOpaqueTokenCanBreakEvenWhenItFitsAloneButNotAfterCjkPrefix", "org.tiqian.layout.LineBreakRepairEngineTest.longOpaqueTokenCanBreakEvenWhenItFitsAloneButNotAfterCjkPrefix", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,111,110,103,79,112,97,113,117,101,84,111,107,101,110,67,97,110,66,114,101,97,107,69,118,101,110,87,104,101,110,73,116,70,105,116,115,65,108,111,110,101,66,117,116,78,111,116,65,102,116,101,114,67,106,107,80,114,101,102,105,120]));
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_blob_test_fits_alone((t).clone()).unwrap();
    });
}

#[test]
fn non_lexical_letter_run_after_cjk_pulls_prefix_onto_loose_line_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.nonLexicalLetterRunAfterCjkPullsPrefixOntoLooseLineWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.nonLexicalLetterRunAfterCjkPullsPrefixOntoLooseLineWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[110,111,110,76,101,120,105,99,97,108,76,101,116,116,101,114,82,117,110,65,102,116,101,114,67,106,107,80,117,108,108,115,80,114,101,102,105,120,79,110,116,111,76,111,111,115,101,76,105,110,101,87,105,116,104,111,117,116,83,121,110,116,104,101,116,105,99,72,121,112,104,101,110]));
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_blob_test_non_lexical((t).clone()).unwrap();
    });
}

#[test]
fn opaque_latin_token_after_cjk_pulls_prefix_onto_loose_line() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.opaqueLatinTokenAfterCjkPullsPrefixOntoLooseLine", "org.tiqian.layout.LineBreakRepairEngineTest.opaqueLatinTokenAfterCjkPullsPrefixOntoLooseLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[111,112,97,113,117,101,76,97,116,105,110,84,111,107,101,110,65,102,116,101,114,67,106,107,80,117,108,108,115,80,114,101,102,105,120,79,110,116,111,76,111,111,115,101,76,105,110,101]));
        let prefix = UString::from("为什么历史是 ").to_ustring();
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UString::from(format!("{}", { let mut __s = UString::new(); __s += prefix.as_ustr(); __s += &(UString::from("abc123def456ghi789")); __s }).as_str()).as_ustr(), 160 as f64, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(NoHyphenator::new())), None).unwrap();
        let first_line_text = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0);
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(first_line_text))) as i32).to_ne_bytes())) > (7), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("first line should carry part of the opaque token instead of stretching only '")); __s += prefix.as_ustr(); __s += &(UString::from("': ")); __s += first_line_text.as_ustr(); __s }).as_str()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.lines[0usize].hyphen_advance == 0 as f64, None).unwrap();
    });
}

#[test]
fn overlong_latin_word_hard_breaks_with_a_hanging_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.overlongLatinWordHardBreaksWithAHangingHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.overlongLatinWordHardBreaksWithAHangingHyphen", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[111,118,101,114,108,111,110,103,76,97,116,105,110,87,111,114,100,72,97,114,100,66,114,101,97,107,115,87,105,116,104,65,72,97,110,103,105,110,103,72,121,112,104,101,110]));
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UStr::new(&[20013,69,110,103,108,105,115,104]), 80 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[69,110,103,108,105,115,104])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[69,110])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[105,115,104])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.lines[0usize].hyphen_advance) > (0 as f64), None).unwrap();
    });
}

#[test]
fn overlong_opaque_latin_token_hard_breaks_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.overlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.overlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[111,118,101,114,108,111,110,103,79,112,97,113,117,101,76,97,116,105,110,84,111,107,101,110,72,97,114,100,66,114,101,97,107,115,87,105,116,104,111,117,116,83,121,110,116,104,101,116,105,99,72,121,112,104,101,110]));
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UStr::new(&[97,98,99,49,50,51,100,101,102,52,53,54,103,104,105,55,56,57]), 96 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), UStr::new(&[97,98,99,49,50,51,100,101,102,52,53,54,103,104,105,55,56,57])), None).unwrap();
        let mut all_within = true;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.lines[usize::try_from(i).unwrap_or(0)].visual_width > (96 as f64) {
                all_within = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_within, None).unwrap();
    });
}

#[test]
fn progressive_technical_break_falls_through_structural_tier_before_overstretching_outside_text() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideText", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideText", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,66,114,101,97,107,70,97,108,108,115,84,104,114,111,117,103,104,83,116,114,117,99,116,117,114,97,108,84,105,101,114,66,101,102,111,114,101,79,118,101,114,115,116,114,101,116,99,104,105,110,103,79,117,116,115,105,100,101,84,101,120,116]));
        let text = UString::from("中 ab/cdefghijk").to_ustring();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 14u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let syllables = SyllableHyphenator::new(vec![2, 4, 6].to_vec());
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 100 as f64, false, Some((*breaker).clone()), Some(Box::new((syllables).clone())), Some(vec![(technical).clone()])).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(7, ((result.lines[0usize]).clone().range).clone().end, Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, result.lines[0usize].hyphen_advance, Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let mut has_note = false;
            for j in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == UString::from("technical-break:Syllable") {
                    has_note = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_note, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += breaker.get_strategy_name().as_ustr(); __s += &(UString::from(": ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&((result.debug).clone().line_decisions[0usize]).clone().notes).as_ustr(); __s }).as_str()))).unwrap();
            let mut adj_index = 4294967295u32;
            for j in 0..match u32::try_from((result.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().justification_decisions[usize::try_from(j).unwrap_or(0)].clone().line_range.clone().start == ((result.lines[0usize]).clone().range).clone().start && (((result.debug).clone().justification_decisions[usize::try_from(j).unwrap_or(0)]).clone().line_range).clone().end == ((result.lines[0usize]).clone().range).clone().end {
                    adj_index = j;
                    break;
                }
            }
            let adjustment = ((result.debug).clone().justification_decisions[usize::try_from(adj_index).unwrap_or(0)]).clone();
            let mut none_in_span = true;
            for j in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let alloc = (adjustment.allocations[usize::try_from(j).unwrap_or(0)]).clone();
                if i32::from_ne_bytes((((alloc.cluster_range).clone().end) as i32).to_ne_bytes()) > (i32::from_ne_bytes((((technical.range).clone().start) as i32).to_ne_bytes())) && (i32::from_ne_bytes((((alloc.cluster_range).clone().end) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((technical.range).clone().end) as i32).to_ne_bytes())) {
                    none_in_span = false;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(none_in_span, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += breaker.get_strategy_name().as_ustr(); __s += &(UString::from(": ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations).as_ustr(); __s }).as_str()))).unwrap();
        }
    });
}

#[test]
fn progressive_technical_break_keeps_cjk_body_unstretched_in_every_strategy() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategy", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategy", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,66,114,101,97,107,75,101,101,112,115,67,106,107,66,111,100,121,85,110,115,116,114,101,116,99,104,101,100,73,110,69,118,101,114,121,83,116,114,97,116,101,103,121]));
        let text = UString::from("中文abcdefghij").to_ustring();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 12u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let syllables = SyllableHyphenator::new(vec![4, 7].to_vec());
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 104 as f64, false, Some((*breaker).clone()), Some(Box::new((syllables).clone())), Some(vec![(technical).clone()])).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(6, ((result.lines[0usize]).clone().range).clone().end, Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, result.lines[0usize].hyphen_advance, Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let mut has_emergency = false;
            for j in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == UString::from("technical-break:Emergency") {
                    has_emergency = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += breaker.get_strategy_name().as_ustr(); __s += &(UString::from(": ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&((result.debug).clone().line_decisions[0usize]).clone().notes).as_ustr(); __s }).as_str()))).unwrap();
            let mut adj_index = 4294967295u32;
            for j in 0..match u32::try_from((result.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().justification_decisions[usize::try_from(j).unwrap_or(0)].clone().line_range.clone().start == ((result.lines[0usize]).clone().range).clone().start && (((result.debug).clone().justification_decisions[usize::try_from(j).unwrap_or(0)]).clone().line_range).clone().end == ((result.lines[0usize]).clone().range).clone().end {
                    adj_index = j;
                    break;
                }
            }
            let adjustment = ((result.debug).clone().justification_decisions[usize::try_from(adj_index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((adjustment.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let mut none_cjk = true;
            for j in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if adjustment.allocations[usize::try_from(j).unwrap_or(0)].clone().kind.to_ustring() == UString::from("CjkInterChar") {
                    none_cjk = false;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(none_cjk, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += breaker.get_strategy_name().as_ustr(); __s += &(UString::from(": ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations).as_ustr(); __s }).as_str()))).unwrap();
            let mut has_emergency_tracking = false;
            for j in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let alloc = (adjustment.allocations[usize::try_from(j).unwrap_or(0)]).clone();
                if alloc.kind.to_ustring() == UString::from("EmergencyGraphemeTracking") && (i32::from_ne_bytes((((alloc.cluster_range).clone().start) as i32).to_ne_bytes())) >= i32::from_ne_bytes((((technical.range).clone().start) as i32).to_ne_bytes()) {
                    has_emergency_tracking = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_emergency_tracking, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += breaker.get_strategy_name().as_ustr(); __s += &(UString::from(": ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations).as_ustr(); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, adjustment.deficit_after, 0.001f64, Some((breaker.get_strategy_name()).to_ustring())).unwrap();
        }
    });
}

#[test]
fn progressive_technical_clean_break_may_not_stretch_earlier_opaque_token() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueToken", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueToken", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,67,108,101,97,110,66,114,101,97,107,77,97,121,78,111,116,83,116,114,101,116,99,104,69,97,114,108,105,101,114,79,112,97,113,117,101,84,111,107,101,110]));
        let text = UString::from("deadbeef1234deadbeef1234 ab.cdEfghijklmnop").to_ustring();
        let terminal_technical_range = TextRange::new(25u32, u_string::unit_count(&(text))).unwrap();
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 300 as f64, false, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(SyllableHyphenator::new(vec![2, 4, 6].to_vec()))), Some(vec![
    (LineBreakSpan::new((terminal_technical_range).clone(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let mut affected_line_index = 4294967295u32;
        for i in 0..match u32::try_from((result.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let dec = ((result.debug).clone().line_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            for j in 0..match u32::try_from(dec.notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if dec.notes[usize::try_from(j).unwrap_or(0)].clone().starts_with(&UString::from("technical-break:")) {
                    affected_line_index = i;
                    break;
                }
            }
            if affected_line_index <= 2147483647 {
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((affected_line_index) <= 2147483647, Some((LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions)).to_ustring())).unwrap();
        let mut has_emergency_note = false;
        for j in 0..match u32::try_from(((result.debug).clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == UString::from("technical-break:Emergency") {
                has_emergency_note = true;
            }
        }
        let mut line_strings: Vec<UString> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_strings.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_emergency_note, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("lines=")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_strings).as_ustr(); __s += &(UString::from(" decisions=")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions).as_ustr(); __s += &(UString::from(" adjustments=")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().justification_decisions).as_ustr(); __s }).as_str()))).unwrap();
        let affected_line = (result.lines[usize::try_from(affected_line_index).unwrap_or(0)]).clone();
        let mut adj_index = 4294967295u32;
        for i in 0..match u32::try_from((result.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let dec = ((result.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if dec.line_range.clone().start == (affected_line.range).clone().start && (dec.line_range).clone().end == (affected_line.range).clone().end {
                adj_index = i;
                break;
            }
        }
        let affected_line_adjustment = ((result.debug).clone().justification_decisions[usize::try_from(adj_index).unwrap_or(0)]).clone();
        let mut emergency_tracking: Vec<JustificationAllocationInfo> = vec![];
        for i in 0..match u32::try_from(affected_line_adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if affected_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)].clone().kind.to_ustring() == UString::from("EmergencyGraphemeTracking") {
                emergency_tracking.push((affected_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((emergency_tracking.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), Some(UString::from(format!("{}", affected_line_adjustment.to_string()).as_str()))).unwrap();
        let mut all_in_technical = true;
        for i in 0..match u32::try_from(emergency_tracking.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes(((((emergency_tracking[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone().start) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((terminal_technical_range.start) as i32).to_ne_bytes())) {
                all_in_technical = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_in_technical, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("a later clean break borrowed tracking from the earlier hash: ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&emergency_tracking).as_ustr(); __s }).as_str()))).unwrap();
    });
}

#[test]
fn progressive_technical_emergency_is_exposed_by_current_line_stretch_not_full_measure() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasure", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasure", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,69,109,101,114,103,101,110,99,121,73,115,69,120,112,111,115,101,100,66,121,67,117,114,114,101,110,116,76,105,110,101,83,116,114,101,116,99,104,78,111,116,70,117,108,108,77,101,97,115,117,114,101]));
        let text = { let mut __s = UString::new(); __s += &(UString::from("Swift 这边是我最有体感的。JSONDecoder 慢是个老问题，")); __s += &(UString::from("SR-6252[36] 那个 issue 里挖出的根因是底层走 NSJSONSerialization ")); __s += &(UString::from("再桥接回 Objective-C，swift_dynamicCast 吃掉大量时间。")); __s };
        let swift_range = TextRange::new(104u32, 121u32).unwrap();
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 579 as f64, false, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(NoHyphenator::new())), Some(vec![
    (LineBreakSpan::new(TextRange::new(16u32, 27u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(67u32, 86u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new((swift_range).clone(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let mut line_texts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_texts.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let mut affected_line_index = 4294967295u32;
        for i in 0..match u32::try_from(line_texts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes(((u_string::find_from(&((line_texts[usize::try_from(i).unwrap_or(0)]).clone()), UString::from("Objective-C").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 {
                affected_line_index = i;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((affected_line_index) <= 2147483647, Some((LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_texts)).to_ustring())).unwrap();
        let affected_line = (result.lines[usize::try_from(affected_line_index).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[101,114,105,97,108,105,122,97,116,105,111,110,32,20877,26725,25509,22238,32,79,98,106,101,99,116,105,118,101,45,67,65292,115,119,105,102,116,95,100,121]), LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), affected_line_index).as_ustr(), None).unwrap();
        let mut has_emergency = false;
        for i in 0..match u32::try_from(((result.debug).clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)].clone().notes[usize::try_from(i).unwrap_or(0)].clone() == UString::from("technical-break:Emergency") {
                has_emergency = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, None).unwrap();
        let mut adj_index = 4294967295u32;
        for i in 0..match u32::try_from((result.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let dec = ((result.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if dec.line_range.clone().start == (affected_line.range).clone().start && (dec.line_range).clone().end == (affected_line.range).clone().end {
                adj_index = i;
                break;
            }
        }
        let mut cjk_stretch = 0.0f64;
        if adj_index <= 2147483647 {
            let allocs = (((result.debug).clone().justification_decisions[usize::try_from(adj_index).unwrap_or(0)]).clone().allocations).clone();
            for i in 0..match u32::try_from(allocs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if allocs[usize::try_from(i).unwrap_or(0)].clone().kind.to_ustring() == UString::from("CjkInterChar") {
                    if allocs[usize::try_from(i).unwrap_or(0)].delta > (cjk_stretch) {
                        cjk_stretch = allocs[usize::try_from(i).unwrap_or(0)].delta;
                    }
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((cjk_stretch) <= 0.001f64, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("current line still stretched CJK body: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(cjk_stretch)); __s }).as_str()))).unwrap();
        let mut has_break_opp = false;
        for i in 0..match u32::try_from((result.debug).clone().break_opportunity_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let opp = ((result.debug).clone().break_opportunity_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if opp.range.clone().start == swift_range.start && (opp.range).clone().end == swift_range.end && opp.tier.as_ref().map_or(false, |v| v == &(UString::from("Emergency").to_ustring())) && (opp.reason).to_ustring() == UString::from("CurrentLineTechnicalEmergencyBreak") {
                has_break_opp = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_break_opp, None).unwrap();
        let mut has_tracking_elig = false;
        for i in 0..match u32::try_from((result.debug).clone().emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let elig = ((result.debug).clone().emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if elig.range.clone().start == swift_range.start && (elig.range).clone().end == swift_range.end && ((elig.reason).to_ustring()).starts_with(&UString::from("CurrentLineTechnicalTierRejection:")) {
                has_tracking_elig = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_tracking_elig, None).unwrap();
    });
}

#[test]
fn progressive_technical_hard_break_overrides_number_run_cohesion() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalHardBreakOverridesNumberRunCohesion", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalHardBreakOverridesNumberRunCohesion", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,72,97,114,100,66,114,101,97,107,79,118,101,114,114,105,100,101,115,78,117,109,98,101,114,82,117,110,67,111,104,101,115,105,111,110]));
        let text = UString::from("aaaaa1234567890bbbb").to_ustring();
        let technical = LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 160 as f64, false, Some((*breaker).clone()), Some(Box::new(NoHyphenator::new())), Some(vec![(technical).clone()])).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[97,97,97,97,97,49,50,51,52,53]), LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), 0).as_ustr(), Some((breaker.get_strategy_name()).to_ustring())).unwrap();
            let mut has_emergency = false;
            for j in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == UString::from("technical-break:Emergency") {
                    has_emergency = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += breaker.get_strategy_name().as_ustr(); __s += &(UString::from(": ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions).as_ustr(); __s }).as_str()))).unwrap();
        }
    });
}

#[test]
fn progressive_technical_structural_break_falls_through_to_emergency_before_tracking() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTracking", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTracking", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,114,111,103,114,101,115,115,105,118,101,84,101,99,104,110,105,99,97,108,83,116,114,117,99,116,117,114,97,108,66,114,101,97,107,70,97,108,108,115,84,104,114,111,117,103,104,84,111,69,109,101,114,103,101,110,99,121,66,101,102,111,114,101,84,114,97,99,107,105,110,103]));
        let text = UString::from("中文ab.cdEfghij").to_ustring();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 13u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 124 as f64, false, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(SyllableHyphenator::new(vec![2, 4, 6].to_vec()))), Some(vec![(technical).clone()])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[20013,25991,97,98,46,99,100]), LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), 0).as_ustr(), None).unwrap();
        let mut has_emergency = false;
        for i in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(i).unwrap_or(0)].clone() == UString::from("technical-break:Emergency") {
                has_emergency = true;
            }
        }
        let mut line_strings: Vec<UString> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_strings.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("lines=")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_strings).as_ustr(); __s += &(UString::from(" decisions=")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions).as_ustr(); __s += &(UString::from(" adjustments=")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().justification_decisions).as_ustr(); __s }).as_str()))).unwrap();
        let mut all_no_hyphen = true;
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance != 0 as f64 {
                all_no_hyphen = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_no_hyphen, None).unwrap();
        let first_line_adjustment = ((result.debug).clone().justification_decisions[0usize]).clone();
        let mut none_cjk = true;
        for i in 0..match u32::try_from(first_line_adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if first_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)].clone().kind.to_ustring() == UString::from("CjkInterChar") {
                none_cjk = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_cjk, None).unwrap();
        let mut has_tracking = false;
        for i in 0..match u32::try_from(first_line_adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let alloc = (first_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if alloc.kind.to_ustring() == UString::from("EmergencyGraphemeTracking") && ((alloc.reason).to_ustring()).starts_with(&UString::from("TerminalTechnicalEmergencyTracking")) {
                has_tracking = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_tracking, None).unwrap();
    });
}

#[test]
fn unbroken_progressive_span_uses_source_space_then_keeps_body_opportunities_available() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.unbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailable", "org.tiqian.layout.LineBreakRepairEngineTest.unbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailable", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[117,110,98,114,111,107,101,110,80,114,111,103,114,101,115,115,105,118,101,83,112,97,110,85,115,101,115,83,111,117,114,99,101,83,112,97,99,101,84,104,101,110,75,101,101,112,115,66,111,100,121,79,112,112,111,114,116,117,110,105,116,105,101,115,65,118,97,105,108,97,98,108,101]));
        let text = UString::from("甲乙ab cd丙丁戊己").to_ustring();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 7u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 129 as f64, false, None, Some(Box::new(NoHyphenator::new())), Some(vec![(technical).clone()])).unwrap();
        let baseline = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_ustr(), 129 as f64, false, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let mut none_tech = true;
        for i in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(i).unwrap_or(0)].clone().starts_with(&UString::from("technical-break:")) {
                none_tech = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_tech, None).unwrap();
        let adjustment = ((result.debug).clone().justification_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((adjustment.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
        let mut baseline_ranges: Vec<TextRange> = vec![];
        for i in 0..match u32::try_from(baseline.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            baseline_ranges.push(((baseline.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        let mut result_ranges: Vec<TextRange> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            result_ranges.push(((result.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_text_range_array(&baseline_ranges, &result_ranges, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, adjustment.deficit_after, 0.001f64, None).unwrap();
        let mut has_whitespace_stretch = false;
        for i in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let alloc = (adjustment.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if alloc.cluster_range.clone().start == 4 && (alloc.cluster_range).clone().end == 5 && (alloc.kind).to_ustring() == UString::from("ProgressiveTechnical") && (alloc.reason).to_ustring() == UString::from("ProgressiveTechnicalWhitespaceStretch") {
                has_whitespace_stretch = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_whitespace_stretch, Some((LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations)).to_ustring())).unwrap();
        let mut has_remaining_body_opp = false;
        for i in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let alloc = (adjustment.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes((((alloc.cluster_range).clone().end) as i32).to_ne_bytes()) <= i32::from_ne_bytes((((technical.range).clone().start) as i32).to_ne_bytes()) || (i32::from_ne_bytes((((alloc.cluster_range).clone().start) as i32).to_ne_bytes())) >= i32::from_ne_bytes((((technical.range).clone().end) as i32).to_ne_bytes()) {
                has_remaining_body_opp = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_remaining_body_opp, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("bounded technical whitespace must not freeze the remaining body opportunities: ")); __s += LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations).as_ustr(); __s }).as_str()))).unwrap();
    });
}

#[test]
fn url_like_latin_token_breaks_at_separators_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.urlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.urlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[117,114,108,76,105,107,101,76,97,116,105,110,84,111,107,101,110,66,114,101,97,107,115,65,116,83,101,112,97,114,97,116,111,114,115,87,105,116,104,111,117,116,83,121,110,116,104,101,116,105,99,72,121,112,104,101,110]));
        let url = UString::from("https://example.com/path/to/abc123def456ghi789").to_ustring();
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(url.as_ustr(), 128 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((result).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((result).clone(), url.as_ustr()), None).unwrap();
        let mut any_ends_with_slash = false;
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring().ends_with(&UString::from("/")) {
                any_ends_with_slash = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any_ends_with_slash, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((result).clone(), UStr::new(&[101,120,97,109,112,108,101,46])), None).unwrap();
        let mut none_forbidden = true;
        for i in 0..match u32::try_from((result.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let dec = ((result.debug).clone().line_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            let repair_decision = dec.repair_decision.clone();
            match &(repair_decision) {
                Some(__option) => {
                    if __option.reason_code.to_ustring() == UString::from("ForbiddenAtLineStart") {
                    none_forbidden = false;
                    }
                }
                None => {
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_forbidden, Some(UString::from("URL separators are LatinText and must not trigger CJK line-start kinsoku"))).unwrap();
    });
}
