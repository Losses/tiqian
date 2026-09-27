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


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestUrlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"allCapsAbbreviationIsNeverBroken");
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(&"INTERNATIONALIZATION中", 128 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"INTERNATIONALIZATION"), None).unwrap();
    });
}

#[test]
fn camel_case_token_breaks_at_the_hump_without_a_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.camelCaseTokenBreaksAtTheHumpWithoutAHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.camelCaseTokenBreaksAtTheHumpWithoutAHyphen", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"camelCaseTokenBreaksAtTheHumpWithoutAHyphen");
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(&"PowerPoint", 128 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"Power"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"Point"), None).unwrap();
    });
}

#[test]
fn greedy_breaker_produces_multiple_lines_when_width_overflows() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.greedyBreakerProducesMultipleLinesWhenWidthOverflows", "org.tiqian.layout.LineBreakRepairEngineTest.greedyBreakerProducesMultipleLinesWhenWidthOverflows", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"greedyBreakerProducesMultipleLinesWhenWidthOverflows");
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(&"中文排版引擎测试", 64 as f64, None, None, None).unwrap();
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
            if r.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().kind.to_string() != "greedy" {
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
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"hyphenatedCompoundBreaksAtExistingHyphenWithoutAddingOne");
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(&"out-of-the-way", 128 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"out-"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"way"), None).unwrap();
        let mut x = String::new();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes((((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes()) <= i32::from_ne_bytes((((r.lines[0usize]).clone().range).clone().end).to_ne_bytes()) {
                x = ((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string();
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((x).ends_with(&"-"), Some((format!("{}{}",
            "line 0 should end at the existing hyphen: ",
            x
        )).to_string())).unwrap();
    });
}

#[test]
fn latin_solidus_breaks_after_slash_without_adding_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.latinSolidusBreaksAfterSlashWithoutAddingHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.latinSolidusBreaksAfterSlashWithoutAddingHyphen", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"latinSolidusBreaksAfterSlashWithoutAddingHyphen");
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(&"TeX/LaTeX", 80 as f64, false, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"TeX/"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"LaTeX"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"TeX/", LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LaTeX", LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 1).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
    });
}

#[test]
fn long_all_caps_opaque_token_hard_breaks_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.longAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.longAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"longAllCapsOpaqueTokenHardBreaksWithoutSyntheticHyphen");
        let s = "QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo".to_string();
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(s.as_str(), 96 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), s.as_str()), None).unwrap();
    });
}

#[test]
fn long_letter_blob_stays_opaque_even_when_tail_looks_hyphenatable() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.longLetterBlobStaysOpaqueEvenWhenTailLooksHyphenatable", "org.tiqian.layout.LineBreakRepairEngineTest.longLetterBlobStaysOpaqueEvenWhenTailLooksHyphenatable", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"longLetterBlobStaysOpaqueEvenWhenTailLooksHyphenatable");
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_blob_test_tail((t).clone()).unwrap();
    });
}

#[test]
fn long_opaque_token_can_break_even_when_it_fits_alone_but_not_after_cjk_prefix() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.longOpaqueTokenCanBreakEvenWhenItFitsAloneButNotAfterCjkPrefix", "org.tiqian.layout.LineBreakRepairEngineTest.longOpaqueTokenCanBreakEvenWhenItFitsAloneButNotAfterCjkPrefix", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"longOpaqueTokenCanBreakEvenWhenItFitsAloneButNotAfterCjkPrefix");
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_blob_test_fits_alone((t).clone()).unwrap();
    });
}

#[test]
fn non_lexical_letter_run_after_cjk_pulls_prefix_onto_loose_line_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.nonLexicalLetterRunAfterCjkPullsPrefixOntoLooseLineWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.nonLexicalLetterRunAfterCjkPullsPrefixOntoLooseLineWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"nonLexicalLetterRunAfterCjkPullsPrefixOntoLooseLineWithoutSyntheticHyphen");
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_blob_test_non_lexical((t).clone()).unwrap();
    });
}

#[test]
fn opaque_latin_token_after_cjk_pulls_prefix_onto_loose_line() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.opaqueLatinTokenAfterCjkPullsPrefixOntoLooseLine", "org.tiqian.layout.LineBreakRepairEngineTest.opaqueLatinTokenAfterCjkPullsPrefixOntoLooseLine", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"opaqueLatinTokenAfterCjkPullsPrefixOntoLooseLine");
        let prefix = "为什么历史是 ".to_string();
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(format!("{}{}",
            prefix,
            "abc123def456ghi789"
        ).as_str(), 160 as f64, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(NoHyphenator::new())), None).unwrap();
        let first_line_text = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0);
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(first_line_text))).to_ne_bytes())) > (7), Some((format!("{}{}{}{}",
            "first line should carry part of the opaque token instead of stretching only '",
            prefix,
            "': ",
            first_line_text
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.lines[0usize].hyphen_advance == 0 as f64, None).unwrap();
    });
}

#[test]
fn overlong_latin_word_hard_breaks_with_a_hanging_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.overlongLatinWordHardBreaksWithAHangingHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.overlongLatinWordHardBreaksWithAHangingHyphen", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"overlongLatinWordHardBreaksWithAHangingHyphen");
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(&"中English", 80 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"English"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"En"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"ish"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.lines[0usize].hyphen_advance) > (0 as f64), None).unwrap();
    });
}

#[test]
fn overlong_opaque_latin_token_hard_breaks_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.overlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.overlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"overlongOpaqueLatinTokenHardBreaksWithoutSyntheticHyphen");
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(&"abc123def456ghi789", 96 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), &"abc123def456ghi789"), None).unwrap();
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
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideText", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideText", ||
{
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"progressiveTechnicalBreakFallsThroughStructuralTierBeforeOverstretchingOutsideText");
        let text = "中 ab/cdefghijk".to_string();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 14u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let syllables = SyllableHyphenator::new(vec![2, 4, 6].to_vec());
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 100 as f64, false, Some((*breaker).clone()), Some(Box::new((syllables).clone())), Some(vec![(technical).clone()])).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(7, ((result.lines[0usize]).clone().range).clone().end, Some((breaker.get_strategy_name()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, result.lines[0usize].hyphen_advance, Some((breaker.get_strategy_name()).to_string())).unwrap();
            let mut has_note = false;
            for j in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == "technical-break:Syllable" {
                    has_note = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_note, Some((format!("{}{}{}",
            breaker.get_strategy_name(),
            ": ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&((result.debug).clone().line_decisions[0usize]).clone().notes)
        )).to_string())).unwrap();
            let mut adj_index = 4294967295u32;
            for j in 0..match u32::try_from((result.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().justification_decisions[usize::try_from(j).unwrap_or(0)].clone().line_range.clone().start == ((result.lines[0usize]).clone().range).clone().start &&
(((result.debug).clone().justification_decisions[usize::try_from(j).unwrap_or(0)]).clone().line_range).clone().end == ((result.lines[0usize]).clone().range).clone().end {
                    adj_index = j;
                    break;
                }
            }
            let adjustment = ((result.debug).clone().justification_decisions[usize::try_from(adj_index).unwrap_or(0)]).clone();
            let mut none_in_span = true;
            for j in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let alloc = (adjustment.allocations[usize::try_from(j).unwrap_or(0)]).clone();
                if i32::from_ne_bytes(((alloc.cluster_range).clone().end).to_ne_bytes()) > (i32::from_ne_bytes(((technical.range).clone().start).to_ne_bytes())) && (i32::from_ne_bytes(((alloc.cluster_range).clone().end).to_ne_bytes())) <
(i32::from_ne_bytes(((technical.range).clone().end).to_ne_bytes())) {
                    none_in_span = false;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(none_in_span, Some((format!("{}{}{}",
            breaker.get_strategy_name(),
            ": ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations)
        )).to_string())).unwrap();
        }
    });
}

#[test]
fn progressive_technical_break_keeps_cjk_body_unstretched_in_every_strategy() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategy", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategy", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"progressiveTechnicalBreakKeepsCjkBodyUnstretchedInEveryStrategy");
        let text = "中文abcdefghij".to_string();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 12u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let syllables = SyllableHyphenator::new(vec![4, 7].to_vec());
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 104 as f64, false, Some((*breaker).clone()), Some(Box::new((syllables).clone())), Some(vec![(technical).clone()])).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_int(6, ((result.lines[0usize]).clone().range).clone().end, Some((breaker.get_strategy_name()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, result.lines[0usize].hyphen_advance, Some((breaker.get_strategy_name()).to_string())).unwrap();
            let mut has_emergency = false;
            for j in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == "technical-break:Emergency" {
                    has_emergency = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, Some((format!("{}{}{}",
            breaker.get_strategy_name(),
            ": ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&((result.debug).clone().line_decisions[0usize]).clone().notes)
        )).to_string())).unwrap();
            let mut adj_index = 4294967295u32;
            for j in 0..match u32::try_from((result.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().justification_decisions[usize::try_from(j).unwrap_or(0)].clone().line_range.clone().start == ((result.lines[0usize]).clone().range).clone().start &&
(((result.debug).clone().justification_decisions[usize::try_from(j).unwrap_or(0)]).clone().line_range).clone().end == ((result.lines[0usize]).clone().range).clone().end {
                    adj_index = j;
                    break;
                }
            }
            let adjustment = ((result.debug).clone().justification_decisions[usize::try_from(adj_index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((adjustment.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), Some((breaker.get_strategy_name()).to_string())).unwrap();
            let mut none_cjk = true;
            for j in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if adjustment.allocations[usize::try_from(j).unwrap_or(0)].clone().kind.to_string() == "CjkInterChar" {
                    none_cjk = false;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(none_cjk, Some((format!("{}{}{}",
            breaker.get_strategy_name(),
            ": ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations)
        )).to_string())).unwrap();
            let mut has_emergency_tracking = false;
            for j in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let alloc = (adjustment.allocations[usize::try_from(j).unwrap_or(0)]).clone();
                if alloc.kind.to_string() == "EmergencyGraphemeTracking" && (i32::from_ne_bytes(((alloc.cluster_range).clone().start).to_ne_bytes())) >= i32::from_ne_bytes(((technical.range).clone().start).to_ne_bytes()) {
                    has_emergency_tracking = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_emergency_tracking, Some((format!("{}{}{}",
            breaker.get_strategy_name(),
            ": ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations)
        )).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, adjustment.deficit_after, 0.001f64, Some((breaker.get_strategy_name()).to_string())).unwrap();
        }
    });
}

#[test]
fn progressive_technical_clean_break_may_not_stretch_earlier_opaque_token() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueToken", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueToken", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"progressiveTechnicalCleanBreakMayNotStretchEarlierOpaqueToken");
        let text = "deadbeef1234deadbeef1234 ab.cdEfghijklmnop".to_string();
        let terminal_technical_range = TextRange::new(25u32, u_string::unit_count(&(text))).unwrap();
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 300 as f64, false, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))),
Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(SyllableHyphenator::new(vec![2, 4, 6].to_vec()))), Some(vec![
    (LineBreakSpan::new((terminal_technical_range).clone(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let mut affected_line_index = 4294967295u32;
        for i in 0..match u32::try_from((result.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let dec = ((result.debug).clone().line_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            for j in 0..match u32::try_from(dec.notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if dec.notes[usize::try_from(j).unwrap_or(0)].clone().starts_with(&"technical-break:") {
                    affected_line_index = i;
                    break;
                }
            }
            if affected_line_index <= 2147483647 {
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((affected_line_index) <= 2147483647, Some((LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions)).to_string())).unwrap();
        let mut has_emergency_note = false;
        for j in 0..match u32::try_from(((result.debug).clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == "technical-break:Emergency" {
                has_emergency_note = true;
            }
        }
        let mut line_strings: Vec<String> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_strings.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_emergency_note, Some((format!("{}{}{}{}{}{}",
            "lines=",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_strings),
            " decisions=",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions),
            " adjustments=",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().justification_decisions)
        )).to_string())).unwrap();
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
            if affected_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)].clone().kind.to_string() == "EmergencyGraphemeTracking" {
                emergency_tracking.push((affected_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((emergency_tracking.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), Some(affected_line_adjustment.to_string())).unwrap();
        let mut all_in_technical = true;
        for i in 0..match u32::try_from(emergency_tracking.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes((((emergency_tracking[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone().start).to_ne_bytes()) < (i32::from_ne_bytes((terminal_technical_range.start).to_ne_bytes())) {
                all_in_technical = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_in_technical, Some((format!("{}{}",
            "a later clean break borrowed tracking from the earlier hash: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&emergency_tracking)
        )).to_string())).unwrap();
    });
}

#[test]
fn progressive_technical_emergency_is_exposed_by_current_line_stretch_not_full_measure() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasure", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasure", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"progressiveTechnicalEmergencyIsExposedByCurrentLineStretchNotFullMeasure");
        let text = format!("{}{}{}",
            "Swift 这边是我最有体感的。JSONDecoder 慢是个老问题，",
            "SR-6252[36] 那个 issue 里挖出的根因是底层走 NSJSONSerialization ",
            "再桥接回 Objective-C，swift_dynamicCast 吃掉大量时间。"
        );
        let swift_range = TextRange::new(104u32, 121u32).unwrap();
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 579 as f64, false, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))),
Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(NoHyphenator::new())), Some(vec![
    (LineBreakSpan::new(TextRange::new(16u32, 27u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(67u32, 86u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new((swift_range).clone(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let mut line_texts: Vec<String> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_texts.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let mut affected_line_index = 4294967295u32;
        for i in 0..match u32::try_from(line_texts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes((u_string::find_from(&(line_texts[usize::try_from(i).unwrap_or(0)]).clone(), "Objective-C", 0)).to_ne_bytes()) <= 2147483647 {
                affected_line_index = i;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((affected_line_index) <= 2147483647, Some((LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_texts)).to_string())).unwrap();
        let affected_line = (result.lines[usize::try_from(affected_line_index).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"erialization 再桥接回 Objective-C，swift_dy", LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), affected_line_index).as_str(), None).unwrap();
        let mut has_emergency = false;
        for i in 0..match u32::try_from(((result.debug).clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[usize::try_from(affected_line_index).unwrap_or(0)].clone().notes[usize::try_from(i).unwrap_or(0)].clone() == "technical-break:Emergency" {
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
                if allocs[usize::try_from(i).unwrap_or(0)].clone().kind.to_string() == "CjkInterChar" {
                    if allocs[usize::try_from(i).unwrap_or(0)].delta > (cjk_stretch) {
                        cjk_stretch = allocs[usize::try_from(i).unwrap_or(0)].delta;
                    }
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((cjk_stretch) <= 0.001f64, Some((format!("{}{}",
            "current line still stretched CJK body: ",
            cjk_stretch
        )).to_string())).unwrap();
        let mut has_break_opp = false;
        for i in 0..match u32::try_from((result.debug).clone().break_opportunity_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let opp = ((result.debug).clone().break_opportunity_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if opp.range.clone().start == swift_range.start && (opp.range).clone().end == swift_range.end && opp.tier.as_ref().map_or(false, |v| v == &("Emergency".to_string())) && (opp.reason).to_string() == "CurrentLineTechnicalEmergencyBreak" {
                has_break_opp = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_break_opp, None).unwrap();
        let mut has_tracking_elig = false;
        for i in 0..match u32::try_from((result.debug).clone().emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let elig = ((result.debug).clone().emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if elig.range.clone().start == swift_range.start && (elig.range).clone().end == swift_range.end && ((elig.reason).to_string()).starts_with(&"CurrentLineTechnicalTierRejection:") {
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
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"progressiveTechnicalHardBreakOverridesNumberRunCohesion");
        let text = "aaaaa1234567890bbbb".to_string();
        let technical = LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let breakers = vec![
    Box::new((GreedyLineBreaker::new(None, None, None, None)).clone()) as Box<dyn LineBreaker>,
    Box::new((LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))).clone()) as Box<dyn LineBreaker>,
    Box::new((ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).unwrap()).clone()) as Box<dyn LineBreaker>,
];
        for breaker in &breakers {
            let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 160 as f64, false, Some((*breaker).clone()), Some(Box::new(NoHyphenator::new())), Some(vec![(technical).clone()])).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(&"aaaaa12345", LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), 0).as_str(), Some((breaker.get_strategy_name()).to_string())).unwrap();
            let mut has_emergency = false;
            for j in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(j).unwrap_or(0)].clone() == "technical-break:Emergency" {
                    has_emergency = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, Some((format!("{}{}{}",
            breaker.get_strategy_name(),
            ": ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions)
        )).to_string())).unwrap();
        }
    });
}

#[test]
fn progressive_technical_structural_break_falls_through_to_emergency_before_tracking() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTracking", "org.tiqian.layout.LineBreakRepairEngineTest.progressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTracking", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"progressiveTechnicalStructuralBreakFallsThroughToEmergencyBeforeTracking");
        let text = "中文ab.cdEfghij".to_string();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 13u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 124 as f64, false, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))),
Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(SyllableHyphenator::new(vec![2, 4, 6].to_vec()))), Some(vec![(technical).clone()])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"中文ab.cd", LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), 0).as_str(), None).unwrap();
        let mut has_emergency = false;
        for i in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(i).unwrap_or(0)].clone() == "technical-break:Emergency" {
                has_emergency = true;
            }
        }
        let mut line_strings: Vec<String> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_strings.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, Some((format!("{}{}{}{}{}{}",
            "lines=",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_strings),
            " decisions=",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().line_decisions),
            " adjustments=",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().justification_decisions)
        )).to_string())).unwrap();
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
            if first_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)].clone().kind.to_string() == "CjkInterChar" {
                none_cjk = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_cjk, None).unwrap();
        let mut has_tracking = false;
        for i in 0..match u32::try_from(first_line_adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let alloc = (first_line_adjustment.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if alloc.kind.to_string() == "EmergencyGraphemeTracking" && ((alloc.reason).to_string()).starts_with(&"TerminalTechnicalEmergencyTracking") {
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
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"unbrokenProgressiveSpanUsesSourceSpaceThenKeepsBodyOpportunitiesAvailable");
        let text = "甲乙ab cd丙丁戊己".to_string();
        let technical = LineBreakSpan::new(TextRange::new(2u32, 7u32).unwrap(), LineBreakPolicy::ProgressiveTechnical);
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 129 as f64, false, None, Some(Box::new(NoHyphenator::new())), Some(vec![(technical).clone()])).unwrap();
        let baseline = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout_with_grid(text.as_str(), 129 as f64, false, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let mut none_tech = true;
        for i in 0..match u32::try_from(((result.debug).clone().line_decisions[0usize]).clone().notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[0usize].clone().notes[usize::try_from(i).unwrap_or(0)].clone().starts_with(&"technical-break:") {
                none_tech = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_tech, None).unwrap();
        let adjustment = ((result.debug).clone().justification_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((adjustment.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
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
            if alloc.cluster_range.clone().start == 4 && (alloc.cluster_range).clone().end == 5 && (alloc.kind).to_string() == "ProgressiveTechnical" && (alloc.reason).to_string() == "ProgressiveTechnicalWhitespaceStretch" {
                has_whitespace_stretch = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_whitespace_stretch, Some((LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations)).to_string())).unwrap();
        let mut has_remaining_body_opp = false;
        for i in 0..match u32::try_from(adjustment.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let alloc = (adjustment.allocations[usize::try_from(i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((alloc.cluster_range).clone().end).to_ne_bytes()) <= i32::from_ne_bytes(((technical.range).clone().start).to_ne_bytes()) || (i32::from_ne_bytes(((alloc.cluster_range).clone().start).to_ne_bytes())) >=
i32::from_ne_bytes(((technical.range).clone().end).to_ne_bytes()) {
                has_remaining_body_opp = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_remaining_body_opp, Some((format!("{}{}",
            "bounded technical whitespace must not freeze the remaining body opportunities: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&adjustment.allocations)
        )).to_string())).unwrap();
    });
}

#[test]
fn url_like_latin_token_breaks_at_separators_without_synthetic_hyphen() {
    testlib::run("org.tiqian.layout.LineBreakRepairEngineTest.urlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphen", "org.tiqian.layout.LineBreakRepairEngineTest.urlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphen", || {
        let mut t = TestTraceRecorder::new("LineBreakRepairEngineTest");
        t.section(&"urlLikeLatinTokenBreaksAtSeparatorsWithoutSyntheticHyphen");
        let url = "https://example.com/path/to/abc123def456ghi789".to_string();
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(url.as_str(), 128 as f64, None, Some(Box::new(NoHyphenator::new())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((result).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((result).clone(), url.as_str()), None).unwrap();
        let mut any_ends_with_slash = false;
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string().ends_with(&"/") {
                any_ends_with_slash = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any_ends_with_slash, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((result).clone(), &"example."), None).unwrap();
        let mut none_forbidden = true;
        for i in 0..match u32::try_from((result.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let dec = ((result.debug).clone().line_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            let repair_decision = dec.repair_decision.clone();
            match &(repair_decision) {
                Some(__option) => {
                    if __option.reason_code.to_string() == "ForbiddenAtLineStart" {
                    none_forbidden = false;
                    }
                }
                None => {
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_forbidden, Some("URL separators are LatinText and must not trigger CJK line-start kinsoku".to_string())).unwrap();
    });
}
