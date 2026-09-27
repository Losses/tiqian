#![cfg(test)]

use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::punctuation_model::PunctuationAnchor;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationInkInput;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::test as testlib;
use std::sync::LazyLock;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault) -> Self {
        match value {
            PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn halt_advance_without_placement_uses_named_profile_fallback() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltAdvanceWithoutPlacementUsesNamedProfileFallback", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltAdvanceWithoutPlacementUsesNamedProfileFallback", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"haltAdvanceWithoutPlacementUsesNamedProfileFallback");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(&"。", Some(PunctuationInkInput::new(16 as f64 as f64, None, Some(7.5f64), None, None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(7.5f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(7.5f64, (a.as_ref().unwrap().halt_advance).unwrap()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8.5f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(&"FontHaltAdvanceWithProfileFallback", (a.as_ref().unwrap().geometry_source).to_string().as_str()).unwrap();
    });
}

#[test]
fn halt_placement_directly_defines_both_compression_sides() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementDirectlyDefinesBothCompressionSides", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementDirectlyDefinesBothCompressionSides", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"haltPlacementDirectlyDefinesBothCompressionSides");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(&"（", Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(5 as f64 as f64, i32::from_ne_bytes((4294967284u32).to_ne_bytes()) as f64 as f64, 11 as f64 as f64, 2 as f64 as f64)),
Some(8 as f64), Some(i32::from_ne_bytes((4294967292u32).to_ne_bytes()) as f64), None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_r(PunctuationAnchor::Center.name().to_string().as_str(), a.as_ref().unwrap().anchor.name().to_string().as_str()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(&"FontHaltFittedBodyCompression", (a.as_ref().unwrap().geometry_source).to_string().as_str()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_nul((a.as_ref().unwrap().halt_validation).clone()).unwrap();
    });
}

#[test]
fn halt_placement_overrides_regional_profile_direction() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementOverridesRegionalProfileDirection", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementOverridesRegionalProfileDirection", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"haltPlacementOverridesRegionalProfileDirection");
        let b = PunctuationAtomBuilder::new(Some(PunctuationGluePlacement::Traditional), None).unwrap();
        let a = b.build(&"。", TextRange::new(0u32, 1u32).unwrap(), PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes((4294967292u32).to_ne_bytes()) as f64
as f64, 7 as f64 as f64, 1 as f64 as f64)), Some(8 as f64), Some(0 as f64), None).unwrap()), None, None).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_nul((a.as_ref().unwrap().halt_validation).clone()).unwrap();
    });
}

#[test]
fn default_ink_caps_a_halt_trim_that_would_cut_into_the_painted_glyph() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.defaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyph", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.defaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyph", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"defaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyph");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(&"（", Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(2 as f64 as f64, i32::from_ne_bytes((4294967284u32).to_ne_bytes()) as f64 as f64, 15 as f64 as f64, 2 as f64 as f64)),
Some(8 as f64), Some(i32::from_ne_bytes((4294967288u32).to_ne_bytes()) as f64), None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(2 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(14 as f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(&"halt-trim-limited-by-default-ink-bounds", ((a.as_ref().unwrap().halt_validation).clone()).as_deref().unwrap_or("")).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(a.as_ref().unwrap().ink_containment_applied, None).unwrap();
    });
}

#[test]
fn equal_halt_advance_falls_through_to_ink_bounds() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.equalHaltAdvanceFallsThroughToInkBounds", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.equalHaltAdvanceFallsThroughToInkBounds", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"equalHaltAdvanceFallsThroughToInkBounds");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(&"，", Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(6 as f64 as f64, i32::from_ne_bytes((4294967292u32).to_ne_bytes()) as f64 as f64, 10 as f64 as f64, 1 as f64 as f64)),
Some(16 as f64), None, None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_nul(a.as_ref().unwrap().halt_advance).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(&"InkBoundsFittedBodyCompression", (a.as_ref().unwrap().geometry_source).to_string().as_str()).unwrap();
    });
}

#[test]
fn microsoft_yahei_centred_comma_compresses_from_both_sides() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiCentredCommaCompressesFromBothSides", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiCentredCommaCompressesFromBothSides", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"microsoftYaheiCentredCommaCompressesFromBothSides");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(&"，", 2048 as f64, 821 as f64, 1130 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, a.as_ref().unwrap().body_width, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(4 as f64, (a.as_ref().unwrap().leading_glue).clone().natural, 0.01f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(4 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural, 0.01f64, None).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_r(PunctuationAnchor::Center.name().to_string().as_str(), a.as_ref().unwrap().anchor.name().to_string().as_str()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
    });
}

#[test]
fn microsoft_yahei_bottom_left_stop_keeps_its_leading_safety_margin() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiBottomLeftStopKeepsItsLeadingSafetyMargin", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiBottomLeftStopKeepsItsLeadingSafetyMargin", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"microsoftYaheiBottomLeftStopKeepsItsLeadingSafetyMargin");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(&"。", 2048 as f64, 131 as f64, 632 as f64).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_r(PunctuationAnchor::Leading.name().to_string().as_str(), a.as_ref().unwrap().anchor.name().to_string().as_str()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
    });
}

#[test]
fn founder_heiti_centred_parentheses_stay_mirror_images() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.founderHeitiCentredParenthesesStayMirrorImages", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.founderHeitiCentredParenthesesStayMirrorImages", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"founderHeitiCentredParenthesesStayMirrorImages");
        let o = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(&"（", 1000 as f64, 456 as f64, 647 as f64).unwrap();
        let c = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(&"）", 1000 as f64, 353 as f64, 544 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((o.as_ref().unwrap().leading_glue).clone().natural, (c.as_ref().unwrap().trailing_glue).clone().natural, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance((o.as_ref().unwrap().trailing_glue).clone().natural, (c.as_ref().unwrap().leading_glue).clone().natural, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((o.as_ref().unwrap().leading_glue).clone().natural) > (0 as f64) && ((o.as_ref().unwrap().trailing_glue).clone().natural) > (0 as f64), None).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, o.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, c.as_ref().unwrap().glyph_inline_shift).unwrap();
    });
}

#[test]
fn underwidth_opening_quote_completes_the_leading_side_of_its_full_width_cell() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.underwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCell", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.underwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCell", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"underwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCell");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(&"“", Some(PunctuationInkInput::new(6 as f64 as f64, Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes((4294967286u32).to_ne_bytes()) as f64 as f64, 5 as f64 as f64, 0 as f64 as f64)),
None, None, None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(16 as f64, a.as_ref().unwrap().advance).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(10 as f64, a.as_ref().unwrap().advance_expansion).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, a.as_ref().unwrap().body_width, 0.001f64, None).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(10 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(&"UnderwidthPunctuationFullWidthBoxPlacement", ((a.as_ref().unwrap().glyph_placement_reason).clone()).as_deref().unwrap_or("")).unwrap();
    });
}

#[test]
fn fixed_half_consumes_measured_sidebearings_instead_of_applying_a_profile_shift() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.fixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShift", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.fixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShift", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"fixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShift");
        let a = (*PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_BUILDER).clone().build(&"《", TextRange::new(0u32, 1u32).unwrap(), PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(6.5f64,
i32::from_ne_bytes((4294967284u32).to_ne_bytes()) as f64 as f64, 15.5f64, 2 as f64 as f64)), None, None, None).unwrap()), None, Some(PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::Kaiming), Some(false)))).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(16 as f64, a.as_ref().unwrap().advance).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(9.5f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(6.5f64, a.as_ref().unwrap().leading_glue_initially_consumed).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().trailing_glue_initially_consumed).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(&"InkBoundsFittedBodyCompressionFixedHalfWidth", (a.as_ref().unwrap().geometry_source).to_string().as_str()).unwrap();
    });
}

#[test]
fn overhang_reduces_compression_capacity_without_moving_ink() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.overhangReducesCompressionCapacityWithoutMovingInk", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.overhangReducesCompressionCapacityWithoutMovingInk", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(&"overhangReducesCompressionCapacityWithoutMovingInk");
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(&"《", Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(6.5f64, i32::from_ne_bytes((4294967284u32).to_ne_bytes()) as f64 as f64, 17 as f64 as f64, 2 as f64 as f64)), None,
None, None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(17 as f64, a.as_ref().unwrap().advance).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(10.5f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(6.5f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(a.as_ref().unwrap().ink_containment_applied, None).unwrap();
    });
}

pub static PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_BUILDER: LazyLock<PunctuationAtomBuilder> = LazyLock::new(|| PunctuationAtomBuilder::new(None, None).unwrap());

#[derive(Clone, Copy)]
pub struct PunctuationAtomBuilderHaltSupport;

impl PunctuationAtomBuilderHaltSupport {
    pub const PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM: f64 = 16.0f64;

    pub fn punctuation_atom_builder_halt_support_s(n: &str) {
        TestTraceRecorder::new("PunctuationAtomBuilderHaltTest").section(n);
    }

    pub fn punctuation_atom_builder_halt_support_f(e: f64, a: f64) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_float(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_q(e: &str, a: &str) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_string(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_r(e: &str, a: &str) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_nul<T: Clone + PartialEq>(v: Option<T>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(v.is_none(), &"-", None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_atom(c: &str, input: Option<PunctuationInkInput>) -> Result<Option<PunctuationAtom>, TextRangeError> {
        return Ok((*PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_BUILDER).clone().build(c, TextRange::new(0u32, 1u32)?, PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, (input).clone(), None, None)?);
    }

    pub fn punctuation_atom_builder_halt_support_unit(c: &str, u: f64, l: f64, rr: f64) -> Result<Option<PunctuationAtom>, TextRangeError> {
        return Ok(PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(c, Some(PunctuationInkInput::new(PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, Some(Rect::new((l / u) *
PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, i32::from_ne_bytes((4294967284u32).to_ne_bytes()) as f64 as f64, (rr / u) * PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, 2 as f64 as f64)), None, None, None)?))?);
    }
}
