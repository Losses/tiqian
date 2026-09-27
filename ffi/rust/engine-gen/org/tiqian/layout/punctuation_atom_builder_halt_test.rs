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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::LazyLock;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestUnderwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCellFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestOverhangReducesCompressionCapacityWithoutMovingInkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestMicrosoftYaheiCentredCommaCompressesFromBothSidesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestMicrosoftYaheiBottomLeftStopKeepsItsLeadingSafetyMarginFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestHaltPlacementOverridesRegionalProfileDirectionFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestHaltPlacementDirectlyDefinesBothCompressionSidesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestHaltAdvanceWithoutPlacementUsesNamedProfileFallbackFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestFounderHeitiCentredParenthesesStayMirrorImagesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestFixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShiftFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestEqualHaltAdvanceFallsThroughToInkBoundsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationAtomBuilderHaltTestDefaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyphFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[104,97,108,116,65,100,118,97,110,99,101,87,105,116,104,111,117,116,80,108,97,99,101,109,101,110,116,85,115,101,115,78,97,109,101,100,80,114,111,102,105,108,101,70,97,108,108,98,97,99,107]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(UStr::new(&[12290]), Some(PunctuationInkInput::new(16 as f64 as f64, None, Some(7.5f64), None, None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(7.5f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(7.5f64, (a.as_ref().unwrap().halt_advance).unwrap()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8.5f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(UStr::new(&[70,111,110,116,72,97,108,116,65,100,118,97,110,99,101,87,105,116,104,80,114,111,102,105,108,101,70,97,108,108,98,97,99,107]), (a.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
    });
}

#[test]
fn halt_placement_directly_defines_both_compression_sides() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementDirectlyDefinesBothCompressionSides", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementDirectlyDefinesBothCompressionSides", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[104,97,108,116,80,108,97,99,101,109,101,110,116,68,105,114,101,99,116,108,121,68,101,102,105,110,101,115,66,111,116,104,67,111,109,112,114,101,115,115,105,111,110,83,105,100,101,115]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(UStr::new(&[65288]), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(5 as f64 as f64, i32::from_ne_bytes(((4294967284u32) as i32).to_ne_bytes()) as f64 as f64, 11 as f64 as f64, 2 as f64 as f64)), Some(8 as f64), Some(i32::from_ne_bytes(((4294967292u32) as i32).to_ne_bytes()) as f64), None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_r(UString::from(PunctuationAnchor::Center.name()).as_ustr(), UString::from(a.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(UStr::new(&[70,111,110,116,72,97,108,116,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110]), (a.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_nul((a.as_ref().unwrap().halt_validation).clone()).unwrap();
    });
}

#[test]
fn halt_placement_overrides_regional_profile_direction() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementOverridesRegionalProfileDirection", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.haltPlacementOverridesRegionalProfileDirection", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[104,97,108,116,80,108,97,99,101,109,101,110,116,79,118,101,114,114,105,100,101,115,82,101,103,105,111,110,97,108,80,114,111,102,105,108,101,68,105,114,101,99,116,105,111,110]));
        let b = PunctuationAtomBuilder::new(Some(PunctuationGluePlacement::Traditional), None).unwrap();
        let a = b.build(UStr::new(&[12290]), TextRange::new(0u32, 1u32).unwrap(), PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967292u32) as i32).to_ne_bytes()) as f64 as f64, 7 as f64 as f64, 1 as f64 as f64)), Some(8 as f64), Some(0 as f64), None).unwrap()), None, None).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_nul((a.as_ref().unwrap().halt_validation).clone()).unwrap();
    });
}

#[test]
fn default_ink_caps_a_halt_trim_that_would_cut_into_the_painted_glyph() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.defaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyph", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.defaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyph", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[100,101,102,97,117,108,116,73,110,107,67,97,112,115,65,72,97,108,116,84,114,105,109,84,104,97,116,87,111,117,108,100,67,117,116,73,110,116,111,84,104,101,80,97,105,110,116,101,100,71,108,121,112,104]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(UStr::new(&[65288]), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(2 as f64 as f64, i32::from_ne_bytes(((4294967284u32) as i32).to_ne_bytes()) as f64 as f64, 15 as f64 as f64, 2 as f64 as f64)), Some(8 as f64), Some(i32::from_ne_bytes(((4294967288u32) as i32).to_ne_bytes()) as f64), None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(2 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(14 as f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(UStr::new(&[104,97,108,116,45,116,114,105,109,45,108,105,109,105,116,101,100,45,98,121,45,100,101,102,97,117,108,116,45,105,110,107,45,98,111,117,110,100,115]), ((a.as_ref().unwrap().halt_validation).clone()).as_deref().unwrap_or(UStr::new(&[]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(a.as_ref().unwrap().ink_containment_applied, None).unwrap();
    });
}

#[test]
fn equal_halt_advance_falls_through_to_ink_bounds() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.equalHaltAdvanceFallsThroughToInkBounds", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.equalHaltAdvanceFallsThroughToInkBounds", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[101,113,117,97,108,72,97,108,116,65,100,118,97,110,99,101,70,97,108,108,115,84,104,114,111,117,103,104,84,111,73,110,107,66,111,117,110,100,115]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(UStr::new(&[65292]), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(6 as f64 as f64, i32::from_ne_bytes(((4294967292u32) as i32).to_ne_bytes()) as f64 as f64, 10 as f64 as f64, 1 as f64 as f64)), Some(16 as f64), None, None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_nul(a.as_ref().unwrap().halt_advance).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(4 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(UStr::new(&[73,110,107,66,111,117,110,100,115,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110]), (a.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
    });
}

#[test]
fn microsoft_yahei_centred_comma_compresses_from_both_sides() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiCentredCommaCompressesFromBothSides", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiCentredCommaCompressesFromBothSides", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[109,105,99,114,111,115,111,102,116,89,97,104,101,105,67,101,110,116,114,101,100,67,111,109,109,97,67,111,109,112,114,101,115,115,101,115,70,114,111,109,66,111,116,104,83,105,100,101,115]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(UStr::new(&[65292]), 2048 as f64, 821 as f64, 1130 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, a.as_ref().unwrap().body_width, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(4 as f64, (a.as_ref().unwrap().leading_glue).clone().natural, 0.01f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(4 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural, 0.01f64, None).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_r(UString::from(PunctuationAnchor::Center.name()).as_ustr(), UString::from(a.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
    });
}

#[test]
fn microsoft_yahei_bottom_left_stop_keeps_its_leading_safety_margin() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiBottomLeftStopKeepsItsLeadingSafetyMargin", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.microsoftYaheiBottomLeftStopKeepsItsLeadingSafetyMargin", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[109,105,99,114,111,115,111,102,116,89,97,104,101,105,66,111,116,116,111,109,76,101,102,116,83,116,111,112,75,101,101,112,115,73,116,115,76,101,97,100,105,110,103,83,97,102,101,116,121,77,97,114,103,105,110]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(UStr::new(&[12290]), 2048 as f64, 131 as f64, 632 as f64).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_r(UString::from(PunctuationAnchor::Leading.name()).as_ustr(), UString::from(a.as_ref().unwrap().anchor.name()).as_ustr()).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
    });
}

#[test]
fn founder_heiti_centred_parentheses_stay_mirror_images() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.founderHeitiCentredParenthesesStayMirrorImages", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.founderHeitiCentredParenthesesStayMirrorImages", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[102,111,117,110,100,101,114,72,101,105,116,105,67,101,110,116,114,101,100,80,97,114,101,110,116,104,101,115,101,115,83,116,97,121,77,105,114,114,111,114,73,109,97,103,101,115]));
        let o = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(UStr::new(&[65288]), 1000 as f64, 456 as f64, 647 as f64).unwrap();
        let c = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_unit(UStr::new(&[65289]), 1000 as f64, 353 as f64, 544 as f64).unwrap();
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
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[117,110,100,101,114,119,105,100,116,104,79,112,101,110,105,110,103,81,117,111,116,101,67,111,109,112,108,101,116,101,115,84,104,101,76,101,97,100,105,110,103,83,105,100,101,79,102,73,116,115,70,117,108,108,87,105,100,116,104,67,101,108,108]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(UStr::new(&[8220]), Some(PunctuationInkInput::new(6 as f64 as f64, Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 5 as f64 as f64, 0 as f64 as f64)), None, None, None).unwrap())).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(16 as f64, a.as_ref().unwrap().advance).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(10 as f64, a.as_ref().unwrap().advance_expansion).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8 as f64, a.as_ref().unwrap().body_width, 0.001f64, None).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(8 as f64, (a.as_ref().unwrap().leading_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, (a.as_ref().unwrap().trailing_glue).clone().natural).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(10 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(UStr::new(&[85,110,100,101,114,119,105,100,116,104,80,117,110,99,116,117,97,116,105,111,110,70,117,108,108,87,105,100,116,104,66,111,120,80,108,97,99,101,109,101,110,116]), ((a.as_ref().unwrap().glyph_placement_reason).clone()).as_deref().unwrap_or(UStr::new(&[]))).unwrap();
    });
}

#[test]
fn fixed_half_consumes_measured_sidebearings_instead_of_applying_a_profile_shift() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.fixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShift", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.fixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShift", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[102,105,120,101,100,72,97,108,102,67,111,110,115,117,109,101,115,77,101,97,115,117,114,101,100,83,105,100,101,98,101,97,114,105,110,103,115,73,110,115,116,101,97,100,79,102,65,112,112,108,121,105,110,103,65,80,114,111,102,105,108,101,83,104,105,102,116]));
        let a = (*PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_BUILDER).clone().build(UStr::new(&[12298]), TextRange::new(0u32, 1u32).unwrap(), PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(6.5f64, i32::from_ne_bytes(((4294967284u32) as i32).to_ne_bytes()) as f64 as f64, 15.5f64, 2 as f64 as f64)), None, None, None).unwrap()), None, Some(PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::Kaiming), Some(false)))).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(16 as f64, a.as_ref().unwrap().advance).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(9.5f64, a.as_ref().unwrap().body_width).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(6.5f64, a.as_ref().unwrap().leading_glue_initially_consumed).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().trailing_glue_initially_consumed).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_f(0 as f64, a.as_ref().unwrap().glyph_inline_shift).unwrap();
        let _ = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_q(UStr::new(&[73,110,107,66,111,117,110,100,115,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110,70,105,120,101,100,72,97,108,102,87,105,100,116,104]), (a.as_ref().unwrap().geometry_source).to_ustring().as_ustr()).unwrap();
    });
}

#[test]
fn overhang_reduces_compression_capacity_without_moving_ink() {
    testlib::run("org.tiqian.layout.PunctuationAtomBuilderHaltTest.overhangReducesCompressionCapacityWithoutMovingInk", "org.tiqian.layout.PunctuationAtomBuilderHaltTest.overhangReducesCompressionCapacityWithoutMovingInk", || {
        PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_s(UStr::new(&[111,118,101,114,104,97,110,103,82,101,100,117,99,101,115,67,111,109,112,114,101,115,115,105,111,110,67,97,112,97,99,105,116,121,87,105,116,104,111,117,116,77,111,118,105,110,103,73,110,107]));
        let a = PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(UStr::new(&[12298]), Some(PunctuationInkInput::new(16 as f64 as f64, Some(Rect::new(6.5f64, i32::from_ne_bytes(((4294967284u32) as i32).to_ne_bytes()) as f64 as f64, 17 as f64 as f64, 2 as f64 as f64)), None, None, None).unwrap())).unwrap();
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

    pub fn punctuation_atom_builder_halt_support_s(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,65,116,111,109,66,117,105,108,100,101,114,72,97,108,116,84,101,115,116]))).section(n);
    }

    pub fn punctuation_atom_builder_halt_support_f(e: f64, a: f64) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_float(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_q(e: &UStr, a: &UStr) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_string(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_r(e: &UStr, a: &UStr) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(e, a, None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_nul<T: Clone + PartialEq>(v: Option<T>) -> Result<(), TracedAssertionsFailFault> {
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(v.is_none(), UStr::new(&[45]), None)?;
        Ok(())
    }

    pub fn punctuation_atom_builder_halt_support_atom(c: &UStr, input: Option<PunctuationInkInput>) -> Result<Option<PunctuationAtom>, TextRangeError> {
        return Ok((*PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_BUILDER).clone().build(c, TextRange::new(0u32, 1u32)?, PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, (input).clone(), None, None)?);
    }

    pub fn punctuation_atom_builder_halt_support_unit(c: &UStr, u: f64, l: f64, rr: f64) -> Result<Option<PunctuationAtom>, TextRangeError> {
        return Ok(PunctuationAtomBuilderHaltSupport::punctuation_atom_builder_halt_support_atom(c, Some(PunctuationInkInput::new(PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, Some(Rect::new((l / u) * PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, i32::from_ne_bytes(((4294967284u32) as i32).to_ne_bytes()) as f64 as f64, (rr / u) * PunctuationAtomBuilderHaltSupport::PUNCTUATION_ATOM_BUILDER_HALT_SUPPORT_EM, 2 as f64 as f64)), None, None, None)?))?);
    }
}
