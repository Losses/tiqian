#![cfg(test)]

use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::punctuation_geometry_ledger::GlueBudget;
use crate::org::tiqian::layout::punctuation_geometry_ledger::GlueCapacity;
use crate::org::tiqian::layout::punctuation_geometry_ledger::PunctuationGeometryLedger;
use crate::org::tiqian::layout::punctuation_geometry_stage::PunctuationGeometryStage;
use crate::org::tiqian::layout::punctuation_model::Glue;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationInkInput;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingAdjustment;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressionResult;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;
use std::sync::LazyLock;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestSpacingPlanAdjustmentsConsumeByTargetAndAnchorFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestSideConsumptionIsCappedAndSkipsNonPositiveAmountsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestLineEdgeTrimConsumesCentredPunctuationOncePerLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestJustificationDeltasAndStructuralChannelsFeedResolvedAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestGlueCapacitiesReportSidesAndPairingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestGeometryWithoutBudgetFallsBackToBodyWidthFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestDecisionInfoListsEveryGeometryWithBudgetsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestClusterIndexRangeFindCoveredClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestBudgetsResolveAdvancesThroughRemainingGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryWithoutGlueEmitsNoDecisionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundarySkipsMandatoryBreakNeighbourFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAtLineEndConsumesTrailingGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault) -> Self {
        match value {
            PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationGeometryLedgerCoverageTestAttachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachmentsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn budgets_resolve_advances_through_remaining_glue() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.budgetsResolveAdvancesThroughRemainingGlue", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.budgetsResolveAdvancesThroughRemainingGlue", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"budgetsResolveAdvancesThroughRemainingGlue");
        let x = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["。".to_string(), "「".to_string(), "中".to_string()]).unwrap();
        let resolved = x.resolve_clusters();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, resolved[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, resolved[1usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, resolved[2usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((resolved[2usize]).clone() == (x.resolve_clusters()[2usize]).clone(), None).unwrap();
    });
}

#[test]
fn glue_capacities_report_sides_and_pairing() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.glueCapacitiesReportSidesAndPairing", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.glueCapacitiesReportSidesAndPairing", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"glueCapacitiesReportSidesAndPairing");
        let m: SortedMapTable<u32, GlueCapacity> = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["。".to_string(), "「".to_string()]).unwrap().glue_capacities();
        let mut ek: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        ek.put(&(1));
        let mut ak: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..u32::from_ne_bytes((m.size()).to_ne_bytes()) {
            ak.put(&(m.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(ek.clone().build(), ak.clone().build(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, m.get(&(1)).as_ref().unwrap().leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, m.get(&(1)).as_ref().unwrap().trailing, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"false", if m.get(&(1)).as_ref().unwrap().paired { "true".to_string() } else { "false".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn side_consumption_is_capped_and_skips_non_positive_amounts() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.sideConsumptionIsCappedAndSkipsNonPositiveAmounts", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.sideConsumptionIsCappedAndSkipsNonPositiveAmounts", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"sideConsumptionIsCappedAndSkipsNonPositiveAmounts");
        let x: SortedMapTable<u32, GlueCapacity> = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["。".to_string(),
"「".to_string()]).unwrap().consume_leading_by_cluster(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![1],
4.0f64)).consume_leading_by_cluster(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![1],
0.0f64)).consume_trailing_by_cluster(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![1],
-1.0f64)).consume_leading_by_cluster(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![99], 8.0f64)).glue_capacities();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, x.get(&(1)).as_ref().unwrap().leading, None).unwrap();
        let y = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["。".to_string(),
"「".to_string()]).unwrap().consume_leading_by_cluster(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![1], 100.0f64));
        let _ = TracedAssertions::traced_assertions_assert_true(!y.glue_capacities().has(&(1)), None).unwrap();
    });
}

#[test]
fn justification_deltas_and_structural_channels_feed_resolved_advance() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.justificationDeltasAndStructuralChannelsFeedResolvedAdvance", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.justificationDeltasAndStructuralChannelsFeedResolvedAdvance", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"justificationDeltasAndStructuralChannelsFeedResolvedAdvance");
        let base = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["「".to_string(), "中".to_string()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, base.resolve_clusters()[0usize].advance, None).unwrap();
        let mut b: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        b.put(&(0), &(1.5f64));
        let justified = base.add_justification_deltas(b.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(17.5f64, justified.resolve_clusters()[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.5f64, justified.to_decision_info()[0usize].justification_delta, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, base.to_decision_info()[0usize].justification_delta, None).unwrap();
        let mut s: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        s.put(&(0), &(2.0f64));
        let spread = base.with_ruby_spread(s.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, spread.resolve_clusters()[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, spread.to_decision_info()[0usize].ruby_spread, None).unwrap();
        let mut r: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        r.put(&(0), &(3.0f64));
        let trimmed = base.with_raw_edge_trims(r.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(13 as f64, trimmed.resolve_clusters()[0usize].advance, None).unwrap();
        let mut r2: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        r2.put(&(0), &(20.0f64));
        let trimmed_twice = trimmed.with_raw_edge_trims(r2.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, trimmed_twice.resolve_clusters()[0usize].advance, None).unwrap();
        let empty: SortedMapTable<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(base.with_ruby_spread((empty).clone()) == base, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(base.with_raw_edge_trims((empty).clone()) == base, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(base.with_inline_box_advances((empty).clone()) == base, None).unwrap();
        let mut r#box: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        r#box.put(&(0), &(4.0f64));
        let boxed = base.with_inline_box_advances(r#box.clone().build());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, boxed.resolve_clusters()[0usize].advance, None).unwrap();
    });
}

#[test]
fn geometry_without_budget_falls_back_to_body_width() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.geometryWithoutBudgetFallsBackToBodyWidth", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.geometryWithoutBudgetFallsBackToBodyWidth", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"geometryWithoutBudgetFallsBackToBodyWidth");
        let x = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["「".to_string(), "中".to_string()]).unwrap();
        let e: SortedMapTable<u32, GlueBudget> = SortedTable::sorted_table_map_builder::<u32, GlueBudget>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build();
        let y = PunctuationGeometryLedger::new(x.natural_clusters.to_vec(), (x.geometries).clone(), (e).clone(), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()),
Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()), Some(PunctuationGeometryLedger::punctuation_geometry_ledger_empty_f()));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, y.resolve_clusters()[0usize].advance, None).unwrap();
        let d = y.add_justification_deltas(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![0], 1 as
f64)).with_ruby_spread(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![0], 2 as f64)).with_raw_edge_trims(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![0], 1 as f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, d.resolve_clusters()[0usize].advance, None).unwrap();
    });
}

#[test]
fn decision_info_lists_every_geometry_with_budgets() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.decisionInfoListsEveryGeometryWithBudgets", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.decisionInfoListsEveryGeometryWithBudgets", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"decisionInfoListsEveryGeometryWithBudgets");
        let i = (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["。".to_string(), "中".to_string()]).unwrap().to_decision_info()[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["。".to_string(), "中".to_string()]).unwrap().to_decision_info().len()) &
0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=1)", (i.range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"。", (i.source_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, i.base_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, i.body_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, i.leading_glue_natural, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, i.trailing_glue_natural, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, i.resolved_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PunctuationGeometryLedger", (i.source).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn spacing_plan_adjustments_consume_by_target_and_anchor() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.spacingPlanAdjustmentsConsumeByTargetAndAnchor", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.spacingPlanAdjustmentsConsumeByTargetAndAnchor", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"spacingPlanAdjustmentsConsumeByTargetAndAnchor");
        let cs = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"「", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"「", 1, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
];
        let mut aa: Vec<PunctuationAtom> = vec![];
        for q in &cs {
            {
                let mut _g = 0u32;
                let _g1 = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((q).clone(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone(), &vec![],
PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
                while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let atom = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    aa.push(atom.clone());
                }
            }
        }
        let stray = PunctuationSpacingAdjustment::new(TextRange::new(90u32, 91u32).unwrap(), TextRange::new(90u32, 91u32).unwrap(), "。", "「", 8 as f64 as f64, 0 as f64 as f64, 8 as f64 as f64, "stray").unwrap();
        let mut ek: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        ek.put(&(0));
        ek.put(&(1));
        let mut ak: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let sm: SortedMapTable<u32, GlueCapacity> = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![(stray).clone()].to_vec()).unwrap()).glue_capacities();
        for i in 0..u32::from_ne_bytes((sm.size()).to_ne_bytes()) {
            ak.put(&(sm.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })));
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(ek.clone().build(), ak.clone().build(), None).unwrap();
        let a = PunctuationSpacingAdjustment::new(TextRange::new(0u32, 1u32).unwrap(), TextRange::new(0u32, 1u32).unwrap(), "「", "「", 8 as f64 as f64, 4 as f64 as f64, 4 as f64 as f64, "leading-side").unwrap();
        let y = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![(a).clone()].to_vec()).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, y.glue_capacities().get(&(0)).as_ref().unwrap().leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, y.glue_capacities().get(&(1)).as_ref().unwrap().leading, None).unwrap();
        let atom = (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone().build(&"·", TextRange::new(0u32, 1u32).unwrap(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, Some(PunctuationInkInput::new(16 as f64 as f64,
Some(Rect::new(2 as f64 as f64, 4 as f64 as f64, 10 as f64 as f64, 12 as f64 as f64)), Some(8 as f64), Some(i32::from_ne_bytes((4294967294u32).to_ne_bytes()) as f64), None).unwrap()), None, None).unwrap();
        let ca = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"·", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"中", 1, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
];
        let ct = PunctuationSpacingAdjustment::new(TextRange::new(0u32, 1u32).unwrap(), TextRange::new(0u32, 1u32).unwrap(), "·", "中", 8 as f64 as f64, 2 as f64 as f64, 6 as f64 as f64, "centred").unwrap();
        let cy = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&ca, &vec![((atom).as_ref().unwrap()).clone()], PunctuationSpacingCompressionResult::new(vec![(ct).clone()].to_vec()).unwrap());
        let cap = cy.glue_capacities().get(&(0));
        let _ = TracedAssertions::traced_assertions_assert_true(cap.as_ref().unwrap().paired, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, cap.as_ref().unwrap().leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, cap.as_ref().unwrap().trailing, None).unwrap();
    });
}

#[test]
fn attached_inline_boundaries_require_alignment_and_run_only_with_attachments() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachments", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachments", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"attachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachments");
        let x = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["。".to_string(), "中".to_string()]).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let x = (x).clone(); Arc::new(move || {
        x.resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None], &vec![], PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let r = x.resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::None], &vec![], PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.trailing_glue_by_cluster.size()).to_ne_bytes()) == 0, None).unwrap();
        let p = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["中".to_string(), "中".to_string()]).unwrap().resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous], &vec![],
PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn attached_inline_boundary_at_line_end_consumes_trailing_glue() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryAtLineEndConsumesTrailingGlue", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryAtLineEndConsumesTrailingGlue", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"attachedInlineBoundaryAtLineEndConsumesTrailingGlue");
        let x = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["」".to_string(), "ref".to_string()]).unwrap();
        let r = x.resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous], &vec![], PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=4)", ((r.decisions[0usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"」", ((r.decisions[0usize]).clone().left_char).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&" ", ((r.decisions[0usize]).clone().right_char).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:line-end", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.decisions[0usize].reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, (r.geometry).clone().resolve_clusters()[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.trailing_glue_by_cluster.size()).to_ne_bytes()) == 0, None).unwrap();
    });
}

#[test]
fn attached_inline_boundary_adjacent_punctuation_halves_the_virtual_glue() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlue", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlue", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"attachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlue");
        let cs = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"」", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"「", 4, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
];
        let mut atoms: Vec<PunctuationAtom> = vec![];
        for q in &cs {
            {
                let mut _g = 0u32;
                let _g1 = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((q).clone(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone(), &vec![],
PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
                while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let a = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    atoms.push(a.clone());
                }
            }
        }
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &atoms, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None], &atoms, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:adjacent-punctuation", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, r.decisions[0usize].natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.decisions[0usize].adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.decisions[0usize].reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=5)", ((r.decisions[0usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let pre = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &atoms,
PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).consume_trailing_by_cluster(PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_budget_at(&vec![0], 4 as f64));
        let b = pre.resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], &atoms, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12 as f64, b.decisions[0usize].natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, b.decisions[0usize].adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((b.trailing_glue_by_cluster.size()).to_ne_bytes()) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, (b.geometry).clone().glue_capacities().get(&(2)).as_ref().unwrap().leading, None).unwrap();
    });
}

#[test]
fn attached_inline_boundary_before_ascii_point_mark_collapses_like_adjacent() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacent", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacent", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"attachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacent");
        let cs = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"」", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&",", 4, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
];
        let atoms = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((cs[0usize]).clone(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone(), &vec![],
PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &atoms, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None], &atoms, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:ascii-point-mark", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.decisions[0usize].natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, r.decisions[0usize].adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&",", ((r.decisions[0usize]).clone().right_char).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn attached_inline_boundary_skips_mandatory_break_neighbour() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundarySkipsMandatoryBreakNeighbour", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundarySkipsMandatoryBreakNeighbour", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"attachedInlineBoundarySkipsMandatoryBreakNeighbour");
        let cs = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"」", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string())).unwrap()).clone(),
    (Cluster::new(TextRange::new(3u32, 4u32).unwrap(), concat!("\n",
""), "mandatory-break", 0 as f64 as f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let atoms = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((cs[0usize]).clone(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone(), &vec![],
PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
        let x = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &atoms, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap());
        let r = x.resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None], &atoms, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:line-end", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn attached_inline_boundary_without_glue_emits_no_decision() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryWithoutGlueEmitsNoDecision", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.attachedInlineBoundaryWithoutGlueEmitsNoDecision", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"attachedInlineBoundaryWithoutGlueEmitsNoDecision");
        let cs = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"「", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"中", 2, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
];
        let mut aa: Vec<PunctuationAtom> = vec![];
        for q in &cs {
            {
                let mut _g = 0u32;
                let _g1 = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((q).clone(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone(), &vec![],
PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
                while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let atom = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    aa.push(atom.clone());
                }
            }
        }
        let r = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &aa, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None], &aa, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let ccs = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"」", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"ref", 1, Some(16 as f64 as f64), Some("latin".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"中", 2, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
];
        let mut caa: Vec<PunctuationAtom> = vec![];
        for q in &ccs {
            {
                let mut _g = 0u32;
                let _g1 = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((q).clone(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone(), &vec![],
PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))).unwrap();
                while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    let atom = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    caa.push(atom.clone());
                }
            }
        }
        let n = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&ccs, &caa, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None], &caa, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:natural", ((n.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, n.decisions[0usize].natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, n.decisions[0usize].adjusted_inner_glue, None).unwrap();
        let mut wide: Vec<PunctuationAtom> = vec![];
        for atom in &caa {
            wide.push(if atom.char.to_string() == "」" { PunctuationAtom::new((atom.range).clone(), (atom.char).to_string().as_str(), atom.punctuation_class, atom.advance, (atom.ink_bounds).clone(), atom.body_width, atom.halt_advance, (atom.halt_validation).clone().clone(),
(atom.leading_glue).clone(), Glue::new((atom.trailing_glue).clone().kind, (atom.trailing_glue).clone().min, 12 as f64 as f64, 12 as f64 as f64, (atom.trailing_glue).clone().priority, (atom.trailing_glue).clone().penalty).unwrap(), atom.anchor,
(atom.geometry_source).to_string().as_str(), atom.policy_body_floor, atom.ink_width, atom.ink_center, atom.ink_containment_body_floor, atom.ink_containment_applied, (atom.ink_bounds_fallback).clone().clone(), atom.advance_expansion, atom.glyph_inline_shift,
(atom.glyph_placement_reason).clone().clone(), Some(atom.leading_glue_initially_consumed), Some(atom.trailing_glue_initially_consumed)).unwrap() } else { (*atom).clone() });
        }
        let w = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&ccs, &wide, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap()).resolve_attached_inline_punctuation_boundaries(&vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None], &wide, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"{1=12}", PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_float_map_text((w.trailing_glue_by_cluster).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(28 as f64, (w.geometry).clone().resolve_clusters()[1usize].advance, None).unwrap();
    });
}

#[test]
fn line_edge_trim_consumes_half_width_at_edges_and_skips_empty_inputs() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.lineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputs", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.lineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputs", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"lineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputs");
        let x = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["」".to_string(), "中".to_string()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.consume_line_edge_glue(&vec![], None).decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let plain = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_ledger(&vec!["中".to_string(), "中".to_string()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((plain.consume_line_edge_glue(&vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_line(0, 1).unwrap()).clone(),
], None).decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.consume_line_edge_glue(&vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_line(1, 0).unwrap()).clone(),
], None).decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let r = x.consume_line_edge_glue(&vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_line(0, 0).unwrap()).clone(),
], None);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"trailing", ((r.decisions[0usize]).clone().side).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.decisions[0usize].trim_amount, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.decisions[0usize].natural_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LineEndHalfWidthPunctuation", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=1)", ((r.decisions[0usize]).clone().cluster_range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, (r.geometry).clone().resolve_clusters()[0usize].advance, None).unwrap();
        let relaxed = x.consume_line_edge_glue(&vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_line(0, 0).unwrap()).clone(),
], Some(false));
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((relaxed.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, (relaxed.geometry).clone().resolve_clusters()[0usize].advance, None).unwrap();
    });
}

#[test]
fn line_edge_trim_consumes_centred_punctuation_once_per_line() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.lineEdgeTrimConsumesCentredPunctuationOncePerLine", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.lineEdgeTrimConsumesCentredPunctuationOncePerLine", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"lineEdgeTrimConsumesCentredPunctuationOncePerLine");
        let atom = (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone().build(&"·", TextRange::new(0u32, 1u32).unwrap(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, Some(PunctuationInkInput::new(16 as f64 as f64,
Some(Rect::new(2 as f64 as f64, 4 as f64 as f64, 10 as f64 as f64, 12 as f64 as f64)), Some(8 as f64), Some(i32::from_ne_bytes((4294967294u32).to_ne_bytes()) as f64), None).unwrap()), None, None).unwrap();
        let x = PunctuationGeometryLedger::punctuation_geometry_ledger_from(&vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"·", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
], &vec![((atom).as_ref().unwrap()).clone()], PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap());
        let r = x.consume_line_edge_glue(&vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_line(0, 0).unwrap()).clone(),
], None);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"both", ((r.decisions[0usize]).clone().side).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, r.decisions[0usize].trim_amount, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, r.decisions[0usize].natural_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LineEndCenteredPunctuationPairedCompression", ((r.decisions[0usize]).clone().reason).to_string().as_str(), None).unwrap();
        let cap = (r.geometry).clone().glue_capacities().get(&(0));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, cap.as_ref().unwrap().leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, cap.as_ref().unwrap().trailing, None).unwrap();
    });
}

#[test]
fn cluster_index_range_find_covered_clusters() {
    testlib::run("org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.clusterIndexRangeFindCoveredClusters", "org.tiqian.layout.PunctuationGeometryLedgerCoverageTest.clusterIndexRangeFindCoveredClusters", || {
        PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_start(&"clusterIndexRangeFindCoveredClusters");
        let cs = vec![
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"中", 0, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"中", 1, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
    (PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c(&"中", 2, Some(16 as f64 as f64), Some("cjk".to_string())).unwrap()).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true(PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&vec![], TextRange::new(0u32, 3u32).unwrap()).is_none(), None).unwrap();
        let a = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&cs, TextRange::new(0u32, 3u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), (a).as_ref().unwrap().clone(), None).unwrap();
        let b = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&cs, TextRange::new(1u32, 2u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(1u32, 1u32), (b).as_ref().unwrap().clone(), None).unwrap();
        let nil = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&cs, TextRange::new(5u32, 6u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", match &(nil) { None => "-".to_string(), Some(__option2) => PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_int_range_text((*__option2).clone()).to_string()
}.as_str(), None).unwrap();
        let d = PunctuationGeometryLedger::punctuation_geometry_ledger_cluster_index_range_for(&cs, TextRange::new(0u32, 1u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), (d).as_ref().unwrap().clone(), None).unwrap();
    });
}

pub static PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER: LazyLock<PunctuationAtomBuilder> = LazyLock::new(|| PunctuationAtomBuilder::new(None, None).unwrap());

#[derive(Clone, Copy)]
pub struct PunctuationGeometryLedgerCoverageSupport;

impl PunctuationGeometryLedgerCoverageSupport {
    pub const PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM: f64 = 16.0f64;

    pub fn punctuation_geometry_ledger_coverage_support_start(n: &str) {
        TestTraceRecorder::new("PunctuationGeometryLedgerCoverageTest").section(n);
    }

    pub fn punctuation_geometry_ledger_coverage_support_c(text: &str, start: u32, advance: Option<f64>, font: Option<String>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(start, u32::wrapping_add(start, u_string::unit_count(&(text))))?, text, (font).as_deref().unwrap_or(""), (advance).unwrap(), Some((text).to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn punctuation_geometry_ledger_coverage_support_ledger(texts: &Vec<String>) -> Result<PunctuationGeometryLedger, TextRangeError> {
        let mut cs: Vec<Cluster> = vec![];
        let mut atoms: Vec<PunctuationAtom> = vec![];
        let mut i = 0u32;
        let mut pos = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((texts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let x = PunctuationGeometryLedgerCoverageSupport::punctuation_geometry_ledger_coverage_support_c((texts[usize::try_from(i).unwrap_or(0)]).clone().as_str(), pos, Some(16 as f64 as f64), Some("cjk".to_string()))?;
            cs.push(x.clone());
            let aa = PunctuationGeometryStage::punctuation_geometry_stage_punctuation_atoms((x).clone(), PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM, (*PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_BUILDER).clone(), &vec![],
PunctuationGluePlacement::MainlandSimplified, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false)))?;
            for a in &aa {
                atoms.push(a.clone());
            }
            pos = u32::wrapping_add(pos, u_string::unit_count(&((texts[usize::try_from(i).unwrap_or(0)]).clone())));
            i = u32::wrapping_add(i, 1);
        }
        return Ok(PunctuationGeometryLedger::punctuation_geometry_ledger_from(&cs, &atoms, PunctuationSpacingCompressor::new()?.compress(&atoms, PunctuationGeometryLedgerCoverageSupport::PUNCTUATION_GEOMETRY_LEDGER_COVERAGE_SUPPORT_EM)?));
    }

    pub fn punctuation_geometry_ledger_coverage_support_float_map_text(m: SortedMapTable<u32, f64>) -> String {
        let mut s = "{".to_string();
        for i in 0..u32::from_ne_bytes((m.size()).to_ne_bytes()) {
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                s += &(", ");
            }
            s += &(format!("{}{}{}",
            crate::runtime::int_text::IntText::int_text(m.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) })),
            "=",
            match m.get(&(m.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }))) { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() }
        ));
        }
        return format!("{}{}",
            s,
            "}"
        );
    }

    pub fn punctuation_geometry_ledger_coverage_support_int_range_text(r: IntRange) -> String {
        let mut s = "[".to_string();
        let mut i = r.start;
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((r.end).to_ne_bytes()) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (i32::from_ne_bytes((r.start).to_ne_bytes())) {
                s += &(", ");
            }
            s += &(i).to_string();
            i = u32::wrapping_add(i, 1);
        }
        return format!("{}{}",
            s,
            "]"
        );
    }

    pub fn punctuation_geometry_ledger_coverage_support_budget_at(entries: &Vec<u32>, value: f64) -> SortedMapTable<u32, f64> {
        let mut b: SortedMapTableBuilder<u32, f64> = SortedTable::sorted_table_map_builder::<u32, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for &k in entries {
            b.put(&(k), &(value));
        }
        return b.clone().build();
    }

    pub fn punctuation_geometry_ledger_coverage_support_line(s: u32, e: u32) -> Result<LineCandidate, TextRangeError> {
        return Ok(LineCandidate::new(IntRange::new(s, e), TextRange::new(s, u32::wrapping_add(e, 1))?, 32 as f64 as f64, 32 as f64 as f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging()))?);
    }
}
