#![cfg(test)]

use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::emergency_grapheme_tracking_test_support::EmergencyGraphemeTrackingTestSupport;
use crate::org::tiqian::layout::emergency_grapheme_tracking_test_support::UniformAdvanceShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestUnannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponentsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestTechnicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControlsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    SupportAssertEqualsTextRangeFault(crate::org::tiqian::layout::emergency_grapheme_tracking_test_support::EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::SupportAssertEqualsTextRangeFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault> for crate::org::tiqian::layout::emergency_grapheme_tracking_test_support::EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::SupportAssertEqualsTextRangeFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::emergency_grapheme_tracking_test_support::EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault> for EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault {
    fn from(value: crate::org::tiqian::layout::emergency_grapheme_tracking_test_support::EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault) -> Self {
        EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::SupportAssertEqualsTextRangeFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestTechnicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergencyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestStandaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestRepeatedPlainTokenGetsNarrowNonLexicalAuthorizationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestRejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCutsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestPlainOpaqueHardBreakKeepsCombiningGraphemeIntactFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestOrdinaryWesternProseIsNeverInferredAsTrackingEligibleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestLongAllCapsWesternWordDoesNotBecomeTrackingEligibleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestHashPieceInsideTechnicalUrlSkipsSyllableClassificationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn hash_piece_inside_technical_url_skips_syllable_classification() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.hashPieceInsideTechnicalUrlSkipsSyllableClassification", "org.tiqian.layout.EmergencyGraphemeTrackingTest.hashPieceInsideTechnicalUrlSkipsSyllableClassification", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[104,97,115,104,80,105,101,99,101,73,110,115,105,100,101,84,101,99,104,110,105,99,97,108,85,114,108,83,107,105,112,115,83,121,108,108,97,98,108,101,67,108,97,115,115,105,102,105,99,97,116,105,111,110]));
        let hash = UString::from("deadbeefcafebabefeedfaceabcdefabcdef").to_ustring();
        let text = { let mut __s = UString::new(); __s += &(UString::from("https://example.com/commit/")); __s += hash.as_ustr(); __s };
        let hash_start = u_string::find_from(&(text), (hash).as_ustr(), 0);
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 192 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let syllable_offsets = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_break_offsets_for_tier(&(result.debug).clone().break_opportunity_decisions, UStr::new(&[83,121,108,108,97,98,108,101]));
        let mut none = true;
        for i in 0..match u32::try_from(syllable_offsets.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes(((syllable_offsets[usize::try_from(i).unwrap_or(0)]) as i32).to_ne_bytes()) > (hash_start) && ({ let v: u32 = syllable_offsets[usize::try_from(i).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u_string::unit_count(&(text))) as i32).to_ne_bytes())) {
                none = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, Some((EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_render_ints(&syllable_offsets)).to_ustring())).unwrap();
        let mut all_zero = true;
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance != 0 as f64 {
                all_zero = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_zero, None).unwrap();
    });
}

#[test]
fn long_all_caps_western_word_does_not_become_tracking_eligible() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.longAllCapsWesternWordDoesNotBecomeTrackingEligible", "org.tiqian.layout.EmergencyGraphemeTrackingTest.longAllCapsWesternWordDoesNotBecomeTrackingEligible", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[108,111,110,103,65,108,108,67,97,112,115,87,101,115,116,101,114,110,87,111,114,100,68,111,101,115,78,111,116,66,101,99,111,109,101,84,114,97,99,107,105,110,103,69,108,105,103,105,98,108,101]));
        let text = UString::from("SUPERCALIFRAGILISTICEXPIALIDOCIOUS").to_ustring();
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 101 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((result.debug).clone().emergency_tracking_eligibility_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let track_allocs = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_allocations_for_kind(&(result.debug).clone().justification_decisions, UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103]));
        let _ = TracedAssertions::traced_assertions_assert_false((i32::from_ne_bytes(((u32::try_from((track_allocs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn ordinary_western_prose_is_never_inferred_as_tracking_eligible() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.ordinaryWesternProseIsNeverInferredAsTrackingEligible", "org.tiqian.layout.EmergencyGraphemeTrackingTest.ordinaryWesternProseIsNeverInferredAsTrackingEligible", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[111,114,100,105,110,97,114,121,87,101,115,116,101,114,110,80,114,111,115,101,73,115,78,101,118,101,114,73,110,102,101,114,114,101,100,65,115,84,114,97,99,107,105,110,103,69,108,105,103,105,98,108,101]));
        let text = UString::from("ordinary Western paragraphs keep their natural word spacing").to_ustring();
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 137 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((result.debug).clone().emergency_tracking_eligibility_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let track_allocs = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_allocations_for_kind(&(result.debug).clone().justification_decisions, UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103]));
        let _ = TracedAssertions::traced_assertions_assert_false((i32::from_ne_bytes(((u32::try_from((track_allocs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
        let mut any_deficit = false;
        for i in 0..match u32::try_from((result.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().justification_decisions[usize::try_from(i).unwrap_or(0)].deficit_after > (0 as f64) {
                any_deficit = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any_deficit, Some(UString::from("ordinary Western lines may remain ragged after bounded word-space adjustment"))).unwrap();
    });
}

#[test]
fn plain_opaque_hard_break_keeps_combining_grapheme_intact() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.plainOpaqueHardBreakKeepsCombiningGraphemeIntact", "org.tiqian.layout.EmergencyGraphemeTrackingTest.plainOpaqueHardBreakKeepsCombiningGraphemeIntact", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[112,108,97,105,110,79,112,97,113,117,101,72,97,114,100,66,114,101,97,107,75,101,101,112,115,67,111,109,98,105,110,105,110,103,71,114,97,112,104,101,109,101,73,110,116,97,99,116]));
        let prefix = UString::from("abc123e").to_ustring();
        let combining_mark = UString::from("́").to_ustring();
        let text = { let mut __s = UString::new(); __s += prefix.as_ustr(); __s += combining_mark.as_ustr(); __s += &(UString::from("def456ghi")); __s };
        let combining_mark_offset = u_string::unit_count(&(prefix));
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 64 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), Some((EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_render_clusters(&result.clusters)).to_ustring())).unwrap();
        let mut none = true;
        let mut ranges: Vec<TextRange> = vec![];
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let r = ((result.clusters[usize::try_from(i).unwrap_or(0)]).clone().range).clone();
            ranges.push(r.clone());
            if r.start == combining_mark_offset || r.end == combining_mark_offset {
                none = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, Some((EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_join_text_ranges(&ranges)).to_ustring())).unwrap();
    });
}

#[test]
fn rejected_letter_digit_structural_offsets_remain_available_as_emergency_cuts() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.rejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCuts", "org.tiqian.layout.EmergencyGraphemeTrackingTest.rejectedLetterDigitStructuralOffsetsRemainAvailableAsEmergencyCuts", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[114,101,106,101,99,116,101,100,76,101,116,116,101,114,68,105,103,105,116,83,116,114,117,99,116,117,114,97,108,79,102,102,115,101,116,115,82,101,109,97,105,110,65,118,97,105,108,97,98,108,101,65,115,69,109,101,114,103,101,110,99,121,67,117,116,115]));
        let text = UString::from("Machine2Machine").to_ustring();
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 120 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let emergency = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_break_offsets_for_tier(&(result.debug).clone().break_opportunity_decisions, UStr::new(&[69,109,101,114,103,101,110,99,121]));
        let mut has7 = false;
        let mut has8 = false;
        for i in 0..match u32::try_from(emergency.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if emergency[usize::try_from(i).unwrap_or(0)] == 7 {
                has7 = true;
            }
            if emergency[usize::try_from(i).unwrap_or(0)] == 8 {
                has8 = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has7, Some((EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_render_ints(&emergency)).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has8, Some((EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_render_ints(&emergency)).to_ustring())).unwrap();
        let mut all_zero = true;
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance != 0 as f64 {
                all_zero = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_zero, None).unwrap();
    });
}

#[test]
fn repeated_plain_token_gets_narrow_non_lexical_authorization() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.repeatedPlainTokenGetsNarrowNonLexicalAuthorization", "org.tiqian.layout.EmergencyGraphemeTrackingTest.repeatedPlainTokenGetsNarrowNonLexicalAuthorization", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[114,101,112,101,97,116,101,100,80,108,97,105,110,84,111,107,101,110,71,101,116,115,78,97,114,114,111,119,78,111,110,76,101,120,105,99,97,108,65,117,116,104,111,114,105,122,97,116,105,111,110]));
        let text = UString::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").to_ustring();
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 101 as f64, None).unwrap();
        let mut any = false;
        for i in 0..match u32::try_from((result.debug).clone().emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((result.debug).clone().emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.range.clone().start == 0 && (d.range).clone().end == u_string::unit_count(&(text)) && (d.reason).to_ustring() == UString::from("LongRepeatedLetterRun") {
                any = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any, None).unwrap();
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.lines[usize::try_from(i).unwrap_or(0)].end_reason == LineEndReason::AutoWrap {
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(101 as f64, result.lines[usize::try_from(i).unwrap_or(0)].visual_width, 0.001f64, None).unwrap();
            }
        }
    });
}

#[test]
fn standalone_technical_hash_uses_tracking_to_fill_every_auto_wrapped_line() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.standaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLine", "org.tiqian.layout.EmergencyGraphemeTrackingTest.standaloneTechnicalHashUsesTrackingToFillEveryAutoWrappedLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[115,116,97,110,100,97,108,111,110,101,84,101,99,104,110,105,99,97,108,72,97,115,104,85,115,101,115,84,114,97,99,107,105,110,103,84,111,70,105,108,108,69,118,101,114,121,65,117,116,111,87,114,97,112,112,101,100,76,105,110,101]));
        let text = UString::from("deadbeefcafebabefeedfaceabcdefabcdef").to_ustring();
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 101 as f64, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let mut auto_count = 0u32;
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.lines[usize::try_from(i).unwrap_or(0)].end_reason == LineEndReason::AutoWrap {
                auto_count = u32::wrapping_add(auto_count, 1);
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((auto_count) as i32).to_ne_bytes())) > (0), None).unwrap();
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.lines[usize::try_from(i).unwrap_or(0)].end_reason == LineEndReason::AutoWrap {
                let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(101 as f64, result.lines[usize::try_from(i).unwrap_or(0)].visual_width, 0.001f64, None).unwrap();
            }
        }
        let mut found = false;
        let allocs = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_allocations_for_kind(&(result.debug).clone().justification_decisions, UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103]));
        for i in 0..match u32::try_from(allocs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if allocs[usize::try_from(i).unwrap_or(0)].clone().reason.to_ustring() == UString::from("TerminalTechnicalEmergencyTracking:ProgressiveTechnicalSpan") {
                found = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(found, None).unwrap();
    });
}

#[test]
fn technical_identifier_relabels_loose_letter_digit_boundary_as_emergency() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.technicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergency", "org.tiqian.layout.EmergencyGraphemeTrackingTest.technicalIdentifierRelabelsLooseLetterDigitBoundaryAsEmergency", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[116,101,99,104,110,105,99,97,108,73,100,101,110,116,105,102,105,101,114,82,101,108,97,98,101,108,115,76,111,111,115,101,76,101,116,116,101,114,68,105,103,105,116,66,111,117,110,100,97,114,121,65,115,69,109,101,114,103,101,110,99,121]));
        let text = UString::from("Machine2Machine").to_ustring();
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout_with_shaper(text.as_ustr(), 85 as f64, Arc::new(Mutex::new(UniformAdvanceShaper::new())), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let _ = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_assert_equals_text_range(TextRange::new(0u32, 8u32).unwrap(), ((result.lines[0usize]).clone().range).clone()).unwrap();
        let mut found_note = false;
        let notes = (((result.debug).clone().line_decisions[0usize]).clone().notes).clone();
        for i in 0..match u32::try_from(notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes(((u_string::find_from(&((notes[usize::try_from(i).unwrap_or(0)]).clone()), UString::from("technical-break:Emergency").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 {
                found_note = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(found_note, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, result.lines[0usize].hyphen_advance, None).unwrap();
    });
}

#[test]
fn technical_tracking_does_not_open_edges_touching_inline_objects_or_zero_width_controls() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.technicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControls", "org.tiqian.layout.EmergencyGraphemeTrackingTest.technicalTrackingDoesNotOpenEdgesTouchingInlineObjectsOrZeroWidthControls", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[116,101,99,104,110,105,99,97,108,84,114,97,99,107,105,110,103,68,111,101,115,78,111,116,79,112,101,110,69,100,103,101,115,84,111,117,99,104,105,110,103,73,110,108,105,110,101,79,98,106,101,99,116,115,79,114,90,101,114,111,87,105,100,116,104,67,111,110,116,114,111,108,115]));
        let object_as = UString::from("aaaaaaaaaaaa").to_ustring();
        let object_bs = UString::from("bbbbbbbbbbbb").to_ustring();
        let object_text = { let mut __s = UString::new(); __s += object_as.as_ustr(); __s += &(UString::from("￼")); __s += object_bs.as_ustr(); __s };
        let object_range = TextRange::new(u_string::unit_count(&(object_as)), u32::wrapping_add(u_string::unit_count(&(object_as)), 1)).unwrap();
        let object_len = u32::wrapping_add(u32::wrapping_add(u_string::unit_count(&(object_as)), 1), u_string::unit_count(&(object_bs)));
        let object_result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout_with_objects(object_text.as_ustr(), 300 as f64, &vec![
    (InlineObjectSpan::new((object_range).clone(), 16 as f64 as f64, 12 as f64 as f64, 4 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
], Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, object_len).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let object_allocations = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_allocations_for_kind(&(object_result.debug).clone().justification_decisions, UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103]));
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((object_allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
        let mut none = true;
        for i in 0..match u32::try_from(object_allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cr = ((object_allocations[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone();
            if cr.end == object_range.start || cr.start == object_range.start && cr.end == object_range.end {
                none = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, Some((EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_render_allocations(&object_allocations)).to_ustring())).unwrap();
        let zero_width_as = UString::from("aaaaaaaaaaaa").to_ustring();
        let zero_width_bs = UString::from("bbbbbbbbbbbb").to_ustring();
        let zero_width_text = { let mut __s = UString::new(); __s += zero_width_as.as_ustr(); __s += &(UString::from("​")); __s += zero_width_bs.as_ustr(); __s };
        let zero_width_range = TextRange::new(u_string::unit_count(&(zero_width_as)), u32::wrapping_add(u_string::unit_count(&(zero_width_as)), 1)).unwrap();
        let zero_width_len = u32::wrapping_add(u32::wrapping_add(u_string::unit_count(&(zero_width_as)), 1), u_string::unit_count(&(zero_width_bs)));
        let zero_width_result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout_with_objects(zero_width_text.as_ustr(), 300 as f64, &vec![], Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, zero_width_len).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
])).unwrap();
        let zero_width_allocations = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_allocations_for_kind(&(zero_width_result.debug).clone().justification_decisions, UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103]));
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((zero_width_allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
        let mut none_z = true;
        for i in 0..match u32::try_from(zero_width_allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cr = ((zero_width_allocations[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone();
            if cr.end == zero_width_range.start || cr.start == zero_width_range.start && cr.end == zero_width_range.end {
                none_z = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_z, Some((EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_render_allocations(&zero_width_allocations)).to_ustring())).unwrap();
    });
}

#[test]
fn unannotated_url_does_not_authorize_tracking_across_ordinary_path_components() {
    testlib::run("org.tiqian.layout.EmergencyGraphemeTrackingTest.unannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponents", "org.tiqian.layout.EmergencyGraphemeTrackingTest.unannotatedUrlDoesNotAuthorizeTrackingAcrossOrdinaryPathComponents", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103,84,101,115,116])));
        t.section(UStr::new(&[117,110,97,110,110,111,116,97,116,101,100,85,114,108,68,111,101,115,78,111,116,65,117,116,104,111,114,105,122,101,84,114,97,99,107,105,110,103,65,99,114,111,115,115,79,114,100,105,110,97,114,121,80,97,116,104,67,111,109,112,111,110,101,110,116,115]));
        let identity = UString::from("abc123def456ghi789").to_ustring();
        let text = { let mut __s = UString::new(); __s += &(UString::from("https://example.com/path/to/")); __s += identity.as_ustr(); __s };
        let identity_start = u_string::find_from(&(text), (identity).as_ustr(), 0);
        let result = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_layout(text.as_ustr(), 160 as f64, None).unwrap();
        let mut actual_ranges: Vec<TextRange> = vec![];
        for i in 0..match u32::try_from((result.debug).clone().emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            actual_ranges.push((((result.debug).clone().emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_text_range_array(&vec![
    (TextRange::new(u32::from_ne_bytes(((identity_start) as u32).to_ne_bytes()), u_string::unit_count(&(text))).unwrap()).clone(),
], &actual_ranges, None).unwrap();
        let track_allocations = EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_allocations_for_kind(&(result.debug).clone().justification_decisions, UStr::new(&[69,109,101,114,103,101,110,99,121,71,114,97,112,104,101,109,101,84,114,97,99,107,105,110,103]));
        let mut all_gte = true;
        for i in 0..match u32::try_from(track_allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if i32::from_ne_bytes(((((track_allocations[usize::try_from(i).unwrap_or(0)]).clone().cluster_range).clone().start) as i32).to_ne_bytes()) < (identity_start) {
                all_gte = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_gte, None).unwrap();
    });
}
