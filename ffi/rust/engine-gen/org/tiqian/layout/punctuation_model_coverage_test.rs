#![cfg(test)]

use crate::org::tiqian::clreq::glue_side::GlueSide;
use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_glue_placements::PunctuationGluePlacements;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::punctuation_model::AdjustmentOpportunity;
use crate::org::tiqian::layout::punctuation_model::Glue;
use crate::org::tiqian::layout::punctuation_model::GlueKind;
use crate::org::tiqian::layout::punctuation_model::PunctuationAnchor;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationInkInput;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingAdjustment;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressionResult;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;
use std::sync::LazyLock;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestUnderwidthGlyphsExpandIntoFullWidthCellByClassSideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestPolicyFallbackSplitsGlueByClassSideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault) -> Self {
        match value {
            PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault) -> Self {
        match value {
            PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault) -> Self {
        match value {
            PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestNonPunctuationCharactersProduceNoAtomFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault) -> Self {
        match value {
            PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault) -> Self {
        match value {
            PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault) -> Self {
        match value {
            PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestInkInputRecordsWhyBoundsAreMissingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault) -> Self {
        match value {
            PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault) -> Self {
        match value {
            PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault) -> Self {
        match value {
            PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestInkBoundsFittedFramePicksTheNarrowestContainingAnchorFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault) -> Self {
        match value {
            PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault) -> Self {
        match value {
            PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault) -> Self {
        match value {
            PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestIndexedBuildRejectsOutOfRangeIndexFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestHaltTrimIsLimitedByInkBoundsAndRecordsWhyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestHaltFromProportionalGlyphIsRejectedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    AtomOrFailFault(crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::AtomOrFailFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault> for crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault {
    fn from(value: PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::AtomOrFailFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault> for PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault {
    fn from(value: crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault) -> Self {
        PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::AtomOrFailFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestHaltFittedCompressionUsesFontMeasurementsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault) -> Self {
        match value {
            PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestHaltAdvanceWithoutPlacementFallsBackToFittedInkOrProfileFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault) -> Self {
        match value {
            PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault) -> Self {
        match value {
            PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault) -> Self {
        match value {
            PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        PunctuationModelCoverageTestGlueRejectsInvertedBoundsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault) -> Self {
        match value {
            PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault) -> Self {
        match value {
            PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault) -> Self {
        match value {
            PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestForcedHalfWidthConnectorsConsumeGlueUpFrontFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault) -> Self {
        match value {
            PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault) -> Self {
        match value {
            PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault) -> Self {
        match value {
            PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestCompressionResultSumsAdjustmentReductionsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    AtomOrFailFault(crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::AtomOrFailFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault> for crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault {
    fn from(value: PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::AtomOrFailFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault> for PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault {
    fn from(value: crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault) -> Self {
        PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::AtomOrFailFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestCjkClosingCompressionRejectsNonMatchingNeighboursFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    AtomOrFailFault(crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::AtomOrFailFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault> for crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault {
    fn from(value: PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::AtomOrFailFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault) -> Self {
        match value {
            PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault> for PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault {
    fn from(value: crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault) -> Self {
        PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::AtomOrFailFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestCjkClosingBeforeAsciiPointMarkCollapsesTrailingGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestAtomOrFailFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestAtomOrFailFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestAtomOrFailFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAtomOrFailFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAtomOrFailFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestAtomOrFailFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestAtomOrFailFault) -> Self {
        match value {
            PunctuationModelCoverageTestAtomOrFailFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAtomOrFailFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestAtomOrFailFault) -> Self {
        match value {
            PunctuationModelCoverageTestAtomOrFailFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAtomOrFailFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestAtomOrFailFault) -> Self {
        match value {
            PunctuationModelCoverageTestAtomOrFailFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestAtomOrFailFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestAtomOrFailFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestAtomOrFailFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestAtomOrFailFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestAtomOrFailFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestAtomOrFailFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestAdjustmentOpportunityCarriesRangeAndGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationTargetsTheWiderSideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    AtomOrFailFault(crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::AtomOrFailFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault> for crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::AtomOrFailFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault> for PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault {
    fn from(value: crate::org::tiqian::layout::punctuation_model_coverage_test::PunctuationModelCoverageTestAtomOrFailFault) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::AtomOrFailFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEmFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault) -> Self {
        match value {
            PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationModelCoverageTestAdjacentPunctuationInnerGlueCollapsesByHalfEmFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn glue_rejects_inverted_bounds() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.glueRejectsInvertedBounds", "org.tiqian.layout.PunctuationModelCoverageTest.glueRejectsInvertedBounds", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[103,108,117,101,82,101,106,101,99,116,115,73,110,118,101,114,116,101,100,66,111,117,110,100,115]));
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        Glue::new(GlueKind::PunctuationTrailing, 2 as f64 as f64, 1 as f64 as f64, 3 as f64 as f64, 0u32, 0u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        Glue::new(GlueKind::PunctuationTrailing, 0 as f64 as f64, 3 as f64 as f64, 1 as f64 as f64, 0u32, 0u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn adjustment_opportunity_carries_range_and_glue() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.adjustmentOpportunityCarriesRangeAndGlue", "org.tiqian.layout.PunctuationModelCoverageTest.adjustmentOpportunityCarriesRangeAndGlue", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[97,100,106,117,115,116,109,101,110,116,79,112,112,111,114,116,117,110,105,116,121,67,97,114,114,105,101,115,82,97,110,103,101,65,110,100,71,108,117,101]));
        let o = AdjustmentOpportunity::new(TextRange::new(1u32, 2u32).unwrap(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_glue(4 as f64).unwrap()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(format!("{}", TextRange::new(1u32, 2u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (o.range).clone().to_string()).as_str()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(4 as f64, (o.glue).clone().natural).unwrap();
    });
}

#[test]
fn compression_result_sums_adjustment_reductions() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.compressionResultSumsAdjustmentReductions", "org.tiqian.layout.PunctuationModelCoverageTest.compressionResultSumsAdjustmentReductions", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[99,111,109,112,114,101,115,115,105,111,110,82,101,115,117,108,116,83,117,109,115,65,100,106,117,115,116,109,101,110,116,82,101,100,117,99,116,105,111,110,115]));
        let r = PunctuationSpacingCompressionResult::new(vec![
    (PunctuationSpacingAdjustment::new(TextRange::new(0u32, 2u32).unwrap(), TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12290])), &(UStr::new(&[12300])), 16 as f64 as f64, 8 as f64 as f64, 8 as f64 as f64, &(UStr::new(&[116,101,115,116,45,97]))).unwrap()).clone(),
    (PunctuationSpacingAdjustment::new(TextRange::new(2u32, 4u32).unwrap(), TextRange::new(2u32, 3u32).unwrap(), &(UStr::new(&[65292])), &(UStr::new(&[12300])), 8 as f64 as f64, 4 as f64 as f64, 4 as f64 as f64, &(UStr::new(&[116,101,115,116,45,98]))).unwrap()).clone(),
].to_vec()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(12 as f64, r.get_total_reduction()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, PunctuationSpacingCompressionResult::new(vec![].to_vec()).unwrap().get_total_reduction()).unwrap();
    });
}

#[test]
fn adjacent_punctuation_inner_glue_collapses_by_half_em() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.adjacentPunctuationInnerGlueCollapsesByHalfEm", "org.tiqian.layout.PunctuationModelCoverageTest.adjacentPunctuationInnerGlueCollapsesByHalfEm", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[97,100,106,97,99,101,110,116,80,117,110,99,116,117,97,116,105,111,110,73,110,110,101,114,71,108,117,101,67,111,108,108,97,112,115,101,115,66,121,72,97,108,102,69,109]));
        let stop = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap();
        let opening = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12300]), Some(1), None).unwrap();
        let r = PunctuationSpacingCompressor::new().unwrap().compress(&vec![((stop).as_ref().unwrap()).clone(), ((opening).as_ref().unwrap()).clone()], PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((r.adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let a = (r.adjustments[0usize]).clone();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(16 as f64, a.natural_inner_glue).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, a.adjusted_inner_glue).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, a.reduction).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(format!("{}", (stop.as_ref().unwrap().range).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (a.reduction_target_range).clone().to_string()).as_str()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[12290]), (a.left_char).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[12300]), (a.right_char).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[99,111,108,108,97,112,115,101,45,97,100,106,97,99,101,110,116,45,112,117,110,99,116,117,97,116,105,111,110,45,105,110,110,101,114,45,103,108,117,101]), (a.reason).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(format!("{}", TextRange::new(0u32, 2u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (a.range).clone().to_string()).as_str()).as_ustr()).unwrap();
    });
}

#[test]
fn adjacent_punctuation_targets_the_wider_side() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.adjacentPunctuationTargetsTheWiderSide", "org.tiqian.layout.PunctuationModelCoverageTest.adjacentPunctuationTargetsTheWiderSide", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[97,100,106,97,99,101,110,116,80,117,110,99,116,117,97,116,105,111,110,84,97,114,103,101,116,115,84,104,101,87,105,100,101,114,83,105,100,101]));
        let stop = PunctuationAtom::new((PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().range).clone(), (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().char).to_ustring().as_ustr(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().punctuation_class, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().advance, (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().ink_bounds).clone(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().body_width, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().halt_advance, (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().halt_validation).clone().clone(), (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().leading_glue).clone(), (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().trailing_glue).clone(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().anchor, (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().geometry_source).to_ustring().as_ustr(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().policy_body_floor, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().ink_width, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().ink_center, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().ink_containment_body_floor, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().ink_containment_applied, (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().ink_bounds_fallback).clone().clone(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().advance_expansion, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().glyph_inline_shift, (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().glyph_placement_reason).clone().clone(), Some(PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12290]), Some(0), None).unwrap().as_ref().unwrap().leading_glue_initially_consumed), Some(8 as f64)).unwrap();
        let opening = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12300]), Some(1), None).unwrap();
        let r = PunctuationSpacingCompressor::new().unwrap().compress(&vec![(stop).clone(), ((opening).as_ref().unwrap()).clone()], PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(format!("{}", (opening.as_ref().unwrap().range).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", ((r.adjustments[0usize]).clone().reduction_target_range).clone().to_string()).as_str()).as_ustr()).unwrap();
    });
}

#[test]
fn adjacent_punctuation_skips_non_adjacent_zero_glue_and_zero_em() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.adjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEm", "org.tiqian.layout.PunctuationModelCoverageTest.adjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEm", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[97,100,106,97,99,101,110,116,80,117,110,99,116,117,97,116,105,111,110,83,107,105,112,115,78,111,110,65,100,106,97,99,101,110,116,90,101,114,111,71,108,117,101,65,110,100,90,101,114,111,69,109]));
        let stop = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12290]), Some(0), None).unwrap();
        let opening = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(2), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationSpacingCompressor::new().unwrap().compress(&vec![(stop).clone(), (opening).clone()], PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let c = PunctuationAtom::new((stop.range).clone(), (stop.char).to_ustring().as_ustr(), stop.punctuation_class, stop.advance, (stop.ink_bounds).clone(), stop.body_width, stop.halt_advance, stop.halt_validation.clone(), (stop.leading_glue).clone(), (stop.trailing_glue).clone(), stop.anchor, (stop.geometry_source).to_ustring().as_ustr(), stop.policy_body_floor, stop.ink_width, stop.ink_center, stop.ink_containment_body_floor, stop.ink_containment_applied, stop.ink_bounds_fallback.clone(), stop.advance_expansion, stop.glyph_inline_shift, stop.glyph_placement_reason.clone(), Some(stop.leading_glue_initially_consumed), Some(8 as f64)).unwrap();
        let ao = PunctuationAtom::new((PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().range).clone(), (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().char).to_ustring().as_ustr(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().punctuation_class, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().advance, (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().ink_bounds).clone(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().body_width, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().halt_advance, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().halt_validation.clone(), (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().leading_glue).clone(), (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().trailing_glue).clone(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().anchor, (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().geometry_source).to_ustring().as_ustr(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().policy_body_floor, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().ink_width, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().ink_center, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().ink_containment_body_floor, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().ink_containment_applied, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().ink_bounds_fallback.clone(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().advance_expansion, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().glyph_inline_shift, PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().glyph_placement_reason.clone(), Some(8 as f64), Some(PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap().trailing_glue_initially_consumed)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationSpacingCompressor::new().unwrap().compress(&vec![(c).clone(), (ao).clone()], PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationSpacingCompressor::new().unwrap().compress(&vec![(stop).clone()], PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((PunctuationSpacingCompressor::new().unwrap().compress(&vec![
    (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12290]), Some(0), None).unwrap()).clone(),
    (PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(1), None).unwrap()).clone(),
], 0 as f64).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn cjk_closing_before_ascii_point_mark_collapses_trailing_glue() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.cjkClosingBeforeAsciiPointMarkCollapsesTrailingGlue", "org.tiqian.layout.PunctuationModelCoverageTest.cjkClosingBeforeAsciiPointMarkCollapsesTrailingGlue", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[99,106,107,67,108,111,115,105,110,103,66,101,102,111,114,101,65,115,99,105,105,80,111,105,110,116,77,97,114,107,67,111,108,108,97,112,115,101,115,84,114,97,105,108,105,110,103,71,108,117,101]));
        let c = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12301]), Some(0), None).unwrap();
        let a = (PunctuationSpacingCompressor::new().unwrap().compress_cjk_closing_before_ascii_point_mark(&vec![(c).clone()], UStr::new(&[12301,44,32,114,101,115,116]), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(format!("{}", TextRange::new(0u32, 2u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (a.range).clone().to_string()).as_str()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(format!("{}", (c.range).clone().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (a.reduction_target_range).clone().to_string()).as_str()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, a.natural_inner_glue).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, a.adjusted_inner_glue).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[12301]), (a.left_char).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[44]), (a.right_char).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[99,111,108,108,97,112,115,101,45,99,106,107,45,99,108,111,115,105,110,103,45,98,101,102,111,114,101,45,97,115,99,105,105,45,112,111,105,110,116,45,109,97,114,107]), (a.reason).to_ustring().as_ustr()).unwrap();
    });
}

#[test]
fn cjk_closing_compression_rejects_non_matching_neighbours() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.cjkClosingCompressionRejectsNonMatchingNeighbours", "org.tiqian.layout.PunctuationModelCoverageTest.cjkClosingCompressionRejectsNonMatchingNeighbours", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[99,106,107,67,108,111,115,105,110,103,67,111,109,112,114,101,115,115,105,111,110,82,101,106,101,99,116,115,78,111,110,77,97,116,99,104,105,110,103,78,101,105,103,104,98,111,117,114,115]));
        let x = PunctuationSpacingCompressor::new().unwrap();
        let o = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12300]), Some(0), None).unwrap();
        let c = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[12301]), Some(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.compress_cjk_closing_before_ascii_point_mark(&vec![(o).clone()], UStr::new(&[12300,44,32,120]), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.compress_cjk_closing_before_ascii_point_mark(&vec![(c).clone()], UStr::new(&[12301]), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.compress_cjk_closing_before_ascii_point_mark(&vec![(c).clone()], UStr::new(&[12301,20013]), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.compress_cjk_closing_before_ascii_point_mark(&vec![
    (PunctuationAtom::new((c.range).clone(), (c.char).to_ustring().as_ustr(), c.punctuation_class, c.advance, (c.ink_bounds).clone(), c.body_width, c.halt_advance, c.halt_validation.clone(), (c.leading_glue).clone(), (c.trailing_glue).clone(), c.anchor, (c.geometry_source).to_ustring().as_ustr(), c.policy_body_floor, c.ink_width, c.ink_center, c.ink_containment_body_floor, c.ink_containment_applied, c.ink_bounds_fallback.clone(), c.advance_expansion, c.glyph_inline_shift, c.glyph_placement_reason.clone(), Some(c.leading_glue_initially_consumed), Some(8 as f64)).unwrap()).clone(),
], UStr::new(&[12301,44,120]), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((x.compress_cjk_closing_before_ascii_point_mark(&vec![(c).clone()], UStr::new(&[12301,44,120]), 0 as f64).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn indexed_build_rejects_out_of_range_index() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.indexedBuildRejectsOutOfRangeIndex", "org.tiqian.layout.PunctuationModelCoverageTest.indexedBuildRejectsOutOfRangeIndex", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[105,110,100,101,120,101,100,66,117,105,108,100,82,101,106,101,99,116,115,79,117,116,79,102,82,97,110,103,101,73,110,100,101,120]));
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul((*PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER).clone().build_at_index(UStr::new(&[65292]), 5, PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap()).unwrap();
        let a = (*PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER).clone().build_at_index(UStr::new(&[65292]), 0, PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nn((a).clone()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(format!("{}", TextRange::new(0u32, 1u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", (a.as_ref().unwrap().range).clone().to_string()).as_str()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[65292]), (a.as_ref().unwrap().char).to_ustring().as_ustr()).unwrap();
    });
}

#[test]
fn non_punctuation_characters_produce_no_atom() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.nonPunctuationCharactersProduceNoAtom", "org.tiqian.layout.PunctuationModelCoverageTest.nonPunctuationCharactersProduceNoAtom", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[110,111,110,80,117,110,99,116,117,97,116,105,111,110,67,104,97,114,97,99,116,101,114,115,80,114,111,100,117,99,101,78,111,65,116,111,109]));
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul(PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[20013]), None, None).unwrap()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul(PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[97]), None, None).unwrap()).unwrap();
    });
}

#[test]
fn policy_fallback_splits_glue_by_class_side() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.policyFallbackSplitsGlueByClassSide", "org.tiqian.layout.PunctuationModelCoverageTest.policyFallbackSplitsGlueByClassSide", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[112,111,108,105,99,121,70,97,108,108,98,97,99,107,83,112,108,105,116,115,71,108,117,101,66,121,67,108,97,115,115,83,105,100,101]));
        let s = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[65292]), None, None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, (s.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, (s.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(PunctuationAnchor::Leading.name()).as_ustr(), UString::from(s.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, s.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[80,114,111,102,105,108,101,71,108,117,101,70,97,108,108,98,97,99,107,87,105,116,104,111,117,116,70,111,110,116,71,101,111,109,101,116,114,121]), (s.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul(s.as_ref().unwrap().halt_advance).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul(s.as_ref().unwrap().ink_containment_body_floor).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(s.as_ref().unwrap().ink_containment_applied, None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul((s.as_ref().unwrap().ink_bounds_fallback).clone()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul((s.as_ref().unwrap().halt_validation).clone()).unwrap();
        let o = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12300]), None, None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, (o.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, (o.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(PunctuationAnchor::Trailing.name()).as_ustr(), UString::from(o.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
        let t = (*PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER).clone().build(UStr::new(&[65292]), TextRange::new(0u32, 1u32).unwrap(), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM, None, Some(PunctuationGluePlacement::Traditional), None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(4 as f64, (t.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(4 as f64, (t.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(PunctuationAnchor::Center.name()).as_ustr(), UString::from(t.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
    });
}

#[test]
fn underwidth_glyphs_expand_into_full_width_cell_by_class_side() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.underwidthGlyphsExpandIntoFullWidthCellByClassSide", "org.tiqian.layout.PunctuationModelCoverageTest.underwidthGlyphsExpandIntoFullWidthCellByClassSide", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[117,110,100,101,114,119,105,100,116,104,71,108,121,112,104,115,69,120,112,97,110,100,73,110,116,111,70,117,108,108,87,105,100,116,104,67,101,108,108,66,121,67,108,97,115,115,83,105,100,101]));
        let o = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12300]), Some(0), Some(PunctuationInkInput::new(8 as f64 as f64, None, None, None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, o.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[85,110,100,101,114,119,105,100,116,104,80,117,110,99,116,117,97,116,105,111,110,70,117,108,108,87,105,100,116,104,66,111,120,80,108,97,99,101,109,101,110,116]), ((o.as_ref().unwrap().glyph_placement_reason).clone()).as_deref().unwrap_or(UStr::new(&[]))).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, o.as_ref().unwrap().advance_expansion).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(16 as f64, o.as_ref().unwrap().advance).unwrap();
        let m = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[183]), Some(0), Some(PunctuationInkInput::new(8 as f64 as f64, None, None, None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(4 as f64, m.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, m.as_ref().unwrap().advance_expansion).unwrap();
        let c = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12301]), Some(0), Some(PunctuationInkInput::new(8 as f64 as f64, None, None, None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, c.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul((c.as_ref().unwrap().glyph_placement_reason).clone()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, c.as_ref().unwrap().advance_expansion).unwrap();
        let e = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12300]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, None, None, None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, e.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul((e.as_ref().unwrap().glyph_placement_reason).clone()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, e.as_ref().unwrap().advance_expansion).unwrap();
    });
}

#[test]
fn halt_fitted_compression_uses_font_measurements() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.haltFittedCompressionUsesFontMeasurements", "org.tiqian.layout.PunctuationModelCoverageTest.haltFittedCompressionUsesFontMeasurements", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[104,97,108,116,70,105,116,116,101,100,67,111,109,112,114,101,115,115,105,111,110,85,115,101,115,70,111,110,116,77,101,97,115,117,114,101,109,101,110,116,115]));
        let a = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom_or_fail(UStr::new(&[183]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(2 as f64 as f64, 4 as f64 as f64, 10 as f64 as f64, 12 as f64 as f64)), Some(8 as f64), Some(i32::from_ne_bytes(((4294967294u32) as i32).to_ne_bytes()) as f64), None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[70,111,110,116,72,97,108,116,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110]), (a.geometry_source).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(2 as f64, (a.leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(6 as f64, (a.trailing_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, a.body_width).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(PunctuationAnchor::Center.name()).as_ustr(), UString::from(a.anchor.name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, *(a.halt_advance).as_ref().unwrap()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul((a.halt_validation).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(a.ink_containment_applied, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(a.ink_containment_body_floor.is_some(), PunctuationModelCoverageSupport::punctuation_model_coverage_support_render_nullable_body_floor(a.ink_containment_body_floor).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn halt_trim_is_limited_by_ink_bounds_and_records_why() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.haltTrimIsLimitedByInkBoundsAndRecordsWhy", "org.tiqian.layout.PunctuationModelCoverageTest.haltTrimIsLimitedByInkBoundsAndRecordsWhy", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[104,97,108,116,84,114,105,109,73,115,76,105,109,105,116,101,100,66,121,73,110,107,66,111,117,110,100,115,65,110,100,82,101,99,111,114,100,115,87,104,121]));
        let a = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[183]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(2 as f64 as f64, 4 as f64 as f64, 14 as f64 as f64, 12 as f64 as f64)), Some(8 as f64), Some(i32::from_ne_bytes(((4294967294u32) as i32).to_ne_bytes()) as f64), None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(2 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(2 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(a.as_ref().unwrap().ink_containment_applied, None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[104,97,108,116,45,116,114,105,109,45,108,105,109,105,116,101,100,45,98,121,45,100,101,102,97,117,108,116,45,105,110,107,45,98,111,117,110,100,115]), ((a.as_ref().unwrap().halt_validation).clone()).as_deref().unwrap_or(UStr::new(&[]))).unwrap();
    });
}

#[test]
fn halt_advance_without_placement_falls_back_to_fitted_ink_or_profile() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.haltAdvanceWithoutPlacementFallsBackToFittedInkOrProfile", "org.tiqian.layout.PunctuationModelCoverageTest.haltAdvanceWithoutPlacementFallsBackToFittedInkOrProfile", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[104,97,108,116,65,100,118,97,110,99,101,87,105,116,104,111,117,116,80,108,97,99,101,109,101,110,116,70,97,108,108,115,66,97,99,107,84,111,70,105,116,116,101,100,73,110,107,79,114,80,114,111,102,105,108,101]));
        let i = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[183]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(8 as f64 as f64, 4 as f64 as f64, 16 as f64 as f64, 12 as f64 as f64)), Some(8 as f64), None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[70,111,110,116,72,97,108,116,65,100,118,97,110,99,101,87,105,116,104,73,110,107,66,111,117,110,100,115,70,105,116,116,101,100,80,108,97,99,101,109,101,110,116]), (i.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
        let p = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[65292]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, None, Some(8 as f64), None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[70,111,110,116,72,97,108,116,65,100,118,97,110,99,101,87,105,116,104,80,114,111,102,105,108,101,70,97,108,108,98,97,99,107]), (p.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, (p.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, (p.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
    });
}

#[test]
fn halt_from_proportional_glyph_is_rejected() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.haltFromProportionalGlyphIsRejected", "org.tiqian.layout.PunctuationModelCoverageTest.haltFromProportionalGlyphIsRejected", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[104,97,108,116,70,114,111,109,80,114,111,112,111,114,116,105,111,110,97,108,71,108,121,112,104,73,115,82,101,106,101,99,116,101,100]));
        let a = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12300]), Some(0), Some(PunctuationInkInput::new(8 as f64 as f64, None, Some(4 as f64), Some(i32::from_ne_bytes(((4294967294u32) as i32).to_ne_bytes()) as f64), None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul(a.as_ref().unwrap().halt_advance).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[85,110,100,101,114,119,105,100,116,104,80,117,110,99,116,117,97,116,105,111,110,70,117,108,108,87,105,100,116,104,66,111,120,80,108,97,99,101,109,101,110,116]), ((a.as_ref().unwrap().glyph_placement_reason).clone()).as_deref().unwrap_or(UStr::new(&[]))).unwrap();
    });
}

#[test]
fn ink_bounds_fitted_frame_picks_the_narrowest_containing_anchor() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.inkBoundsFittedFramePicksTheNarrowestContainingAnchor", "org.tiqian.layout.PunctuationModelCoverageTest.inkBoundsFittedFramePicksTheNarrowestContainingAnchor", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[105,110,107,66,111,117,110,100,115,70,105,116,116,101,100,70,114,97,109,101,80,105,99,107,115,84,104,101,78,97,114,114,111,119,101,115,116,67,111,110,116,97,105,110,105,110,103,65,110,99,104,111,114]));
        let r = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12301]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(8 as f64 as f64, 4 as f64 as f64, 16 as f64 as f64, 12 as f64 as f64)), None, None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(PunctuationAnchor::Trailing.name()).as_ustr(), UString::from(r.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, (r.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, (r.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[73,110,107,66,111,117,110,100,115,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110]), (r.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(r.as_ref().unwrap().ink_containment_applied, None).unwrap();
        let l = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12300]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(0 as f64 as f64, 4 as f64 as f64, 8 as f64 as f64, 12 as f64 as f64)), None, None, None).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(PunctuationAnchor::Leading.name()).as_ustr(), UString::from(l.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, (l.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, (l.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let w = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[12301]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(1 as f64 as f64, 4 as f64 as f64, 15 as f64 as f64, 12 as f64 as f64)), None, None, None).unwrap())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(w.as_ref().unwrap().ink_containment_applied, None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(14 as f64, (w.as_ref().unwrap().ink_containment_body_floor).unwrap()).unwrap();
    });
}

#[test]
fn forced_half_width_connectors_consume_glue_up_front() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.forcedHalfWidthConnectorsConsumeGlueUpFront", "org.tiqian.layout.PunctuationModelCoverageTest.forcedHalfWidthConnectorsConsumeGlueUpFront", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[102,111,114,99,101,100,72,97,108,102,87,105,100,116,104,67,111,110,110,101,99,116,111,114,115,67,111,110,115,117,109,101,71,108,117,101,85,112,70,114,111,110,116]));
        let h = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[45]), None, None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, h.as_ref().unwrap().advance).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, h.as_ref().unwrap().body_width).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((h.as_ref().unwrap().geometry_source).to_ustring()).ends_with(&UString::from("FixedHalfWidth")), None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, h.as_ref().unwrap().leading_glue_initially_consumed).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(0 as f64, h.as_ref().unwrap().trailing_glue_initially_consumed).unwrap();
        let d = (*PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER).clone().build(UStr::new(&[183]), TextRange::new(0u32, 1u32).unwrap(), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM, None, None, Some(PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(true)))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d.as_ref().unwrap().geometry_source).to_ustring()).ends_with(&UString::from("FixedHalfWidth")), None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(4 as f64, d.as_ref().unwrap().leading_glue_initially_consumed).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(4 as f64, d.as_ref().unwrap().trailing_glue_initially_consumed).unwrap();
        let k = (*PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER).clone().build(UStr::new(&[65292]), TextRange::new(0u32, 1u32).unwrap(), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM, None, None, Some(PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::Kaiming), Some(false)))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((k.as_ref().unwrap().geometry_source).to_ustring()).ends_with(&UString::from("FixedHalfWidth")), None).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(8 as f64, k.as_ref().unwrap().trailing_glue_initially_consumed).unwrap();
        let ks = (*PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER).clone().build(UStr::new(&[12290]), TextRange::new(0u32, 1u32).unwrap(), PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM, None, None, Some(PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::Kaiming), Some(false)))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(((ks.as_ref().unwrap().geometry_source).to_ustring()).ends_with(&UString::from("FixedHalfWidth")), None).unwrap();
    });
}

#[test]
fn ink_input_records_why_bounds_are_missing() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.inkInputRecordsWhyBoundsAreMissing", "org.tiqian.layout.PunctuationModelCoverageTest.inkInputRecordsWhyBoundsAreMissing", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[105,110,107,73,110,112,117,116,82,101,99,111,114,100,115,87,104,121,66,111,117,110,100,115,65,114,101,77,105,115,115,105,110,103]));
        let a = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[65292]), Some(0), Some(PunctuationInkInput::new(16 as f64 as f64, None, None, None, Some(UString::from("shaper-no-ink-bounds"))).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[115,104,97,112,101,114,45,110,111,45,105,110,107,45,98,111,117,110,100,115]), ((a.as_ref().unwrap().ink_bounds_fallback).clone()).as_deref().unwrap_or(UStr::new(&[]))).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_nul((a.as_ref().unwrap().ink_bounds).clone()).unwrap();
        let b = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(UStr::new(&[65292]), Some(0), Some(PunctuationInkInput::new(0 as f64 as f64, None, None, None, Some(UString::from("glyph-cluster-mapping-ambiguous"))).unwrap())).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqs(UStr::new(&[103,108,121,112,104,45,99,108,117,115,116,101,114,45,109,97,112,112,105,110,103,45,97,109,98,105,103,117,111,117,115]), ((b.as_ref().unwrap().ink_bounds_fallback).clone()).as_deref().unwrap_or(UStr::new(&[]))).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqf(16 as f64, b.as_ref().unwrap().advance).unwrap();
    });
}

#[test]
fn glue_side_for_mainland_simplified_maps_classes_to_sides() {
    testlib::run("org.tiqian.layout.PunctuationModelCoverageTest.glueSideForMainlandSimplifiedMapsClassesToSides", "org.tiqian.layout.PunctuationModelCoverageTest.glueSideForMainlandSimplifiedMapsClassesToSides", || {
        PunctuationModelCoverageSupport::punctuation_model_coverage_support_start(UStr::new(&[103,108,117,101,83,105,100,101,70,111,114,77,97,105,110,108,97,110,100,83,105,109,112,108,105,102,105,101,100,77,97,112,115,67,108,97,115,115,101,115,84,111,83,105,100,101,115]));
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(GlueSide::LeadingOnly.name()).as_ustr(), UString::from(PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(PunctuationGluePlacement::MainlandSimplified, PunctuationClass::Opening).name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(GlueSide::TrailingOnly.name()).as_ustr(), UString::from(PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(PunctuationGluePlacement::MainlandSimplified, PunctuationClass::Closing).name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(GlueSide::TrailingOnly.name()).as_ustr(), UString::from(PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(PunctuationGluePlacement::MainlandSimplified, PunctuationClass::PauseOrStop).name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(GlueSide::BothSides.name()).as_ustr(), UString::from(PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(PunctuationGluePlacement::MainlandSimplified, PunctuationClass::MiddleDot).name()).as_ustr()).unwrap();
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_eqr(UString::from(GlueSide::BothSides.name()).as_ustr(), UString::from(PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(PunctuationGluePlacement::Traditional, PunctuationClass::Opening).name()).as_ustr()).unwrap();
    });
}

pub static PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER: LazyLock<PunctuationAtomBuilder> = LazyLock::new(|| PunctuationAtomBuilder::new(None, None).unwrap());

#[derive(Clone, Copy)]
pub struct PunctuationModelCoverageSupport;

impl PunctuationModelCoverageSupport {
    pub const PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM: f64 = 16.0f64;

    pub fn punctuation_model_coverage_support_start(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,77,111,100,101,108,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }

    pub fn punctuation_model_coverage_support_glue(n: f64) -> Result<Glue, TextRangeError> {
        return Ok(Glue::new(GlueKind::PunctuationTrailing, 0 as f64 as f64, n, n, 0u32, 0u32)?);
    }

    pub fn punctuation_model_coverage_support_atom(c: &UStr, s: Option<u32>, ink: Option<PunctuationInkInput>) -> Result<Option<PunctuationAtom>, TextRangeError> {
        let i = match &(s) { None => 0, Some(__option) => *__option };
        return Ok((*PUNCTUATION_MODEL_COVERAGE_SUPPORT_BUILDER).clone().build(c, TextRange::new(i, u32::wrapping_add(i, 1))?, PunctuationModelCoverageSupport::PUNCTUATION_MODEL_COVERAGE_SUPPORT_EM, (ink).clone(), None, None)?);
    }

    pub fn punctuation_model_coverage_support_atom_or_fail(c: &UStr, s: Option<u32>, ink: Option<PunctuationInkInput>) -> Result<PunctuationAtom, PunctuationModelCoverageTestAtomOrFailFault> {
        let a = PunctuationModelCoverageSupport::punctuation_model_coverage_support_atom(c, s, (ink).clone()).map_err(|e| PunctuationModelCoverageTestAtomOrFailFault::TextRangeErrorFault(e))?;
        let _ = PunctuationModelCoverageSupport::punctuation_model_coverage_support_reject_missing_atom((a).clone()).map_err(|e| PunctuationModelCoverageTestAtomOrFailFault::TracedAssertionsFailFaultFault(e))?;
        return Ok((a).unwrap());
    }

    pub fn punctuation_model_coverage_support_reject_missing_atom(v: Option<PunctuationAtom>) -> Result<(), TracedAssertionsFailFault> {
        if v.is_none() {
            let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(false, UStr::new(&[45]), None)?;
        }
        Ok(())
    }

    pub fn punctuation_model_coverage_support_nul<T: Clone + PartialEq>(v: Option<T>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(v.is_none(), UStr::new(&[45]), None)?;
        Ok(())
    }

    pub fn punctuation_model_coverage_support_nn(v: Option<PunctuationAtom>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(v.is_some(), match &(v) { None => UString::from("-"), Some(__option3) => PunctuationModelCoverageSupport::punctuation_model_coverage_support_render_punctuation_atom(((*__option3).clone()).clone()).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?.to_ustring() }.as_ustr(), None)?;
        Ok(())
    }

    pub fn punctuation_model_coverage_support_render_nullable_body_floor(v: Option<f64>) -> Result<UString, UStringFault> {
        return Ok(match &(v) { None => UString::from("null"), Some(__option4) => PunctuationModelCoverageSupport::punctuation_model_coverage_support_render_body_floor_text(*__option4)?.to_ustring() });
    }

    pub fn punctuation_model_coverage_support_render_body_floor_text(v: f64) -> Result<UString, UStringFault> {
        return Ok(UString::from(format!("{}", TestTraceRender::test_trace_render_cap(UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()).as_ustr())?).as_str()));
    }

    pub fn punctuation_model_coverage_support_render_punctuation_atom(v: PunctuationAtom) -> Result<UString, UStringFault> {
        return Ok(TestTraceRender::test_trace_render_cap(UString::from(format!("{}", v.to_string()).as_str()).as_ustr())?);
    }

    pub fn punctuation_model_coverage_support_eqf(e: f64, a: f64) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_float(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_model_coverage_support_eqs(e: &UStr, a: &UStr) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_string(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_model_coverage_support_eqr(e: &UStr, a: &UStr) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(e, a, None)?;
        Ok(())
    }
}
